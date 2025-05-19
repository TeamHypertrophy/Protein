/*
______          _       _
| ___ \        | |     (_)
| |_/ / __ ___ | |_ ___ _ _ __
|  __/ '__/ _ \| __/ _ \ | '_ \
| |  | | | (_) | ||  __/ | | | |
\_|  |_|  \___/ \__\___|_|_| |_|

        Made with ❤️
*/

// Rocket
use rocket_client_addr::ClientRealAddr;
use user_agent_parser::OS;
use rocket::{
    State, get,
    http::ContentType,
    post,
    response::status,
    serde::{
        json::{Json, Value, json},
        uuid::Uuid,
    },
};
// Validation
use validator::Validate;
// Email Templates
use askama::Template;

// Protein
use crate::{
    auth::api::API,
    auth::rate_limit::RateLimit,
    cache::redis::{Cache, Redis},
    db,
    db::DB,
    errors::Error,
    models::keys::APIKey,
    models::user::{ForgotPassword, LoginUser, NewUser, Password, ProteinUser, UpdateUser, User},
    utils::admin::Config,
    utils::email,
    utils::email::Email,
    utils::password,
    utils::templates::{
        AccountDeleted, EmailVerified, Login, MFACode, MFADisabled, MFAVerification, PasswordReset,
        RequestPasswordReset, Signup, Success, UpdatedPassword,
    },
    utils::webhook,
    utils::webhook::Webhook,
};

#[get("/?<user_id>", format = "application/json")]
pub async fn get_user(
    _r: RateLimit<'_>,
    _auth: API,
    user_id: Uuid,
    pool: &State<DB>,
    redis: &State<Redis>,
) -> Result<Json<User>, Error> {
    // Check Cache
    let cache: Value = Cache::get(redis, "user", user_id).await?;

    // If Cache is Null, Fetch From Database
    if cache.is_null() {
        // Crate Database Connection
        let connection = &mut db::get(pool).await?;

        // Grab User
        let user = User::find(user_id, connection).await?;

        // Set User in Cache
        Cache::set(redis, "user", user_id, Cache::serialize(&user)?).await?;

        Ok(Json(user))
    } else {
        // Deserialize User
        let user: User = Cache::deserialize(cache)?;

        Ok(Json(user))
    }
}

#[get("/all", format = "application/json")]
pub async fn get_all_users(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
) -> Result<Json<Vec<User>>, Error> {
    // Creating Database Connection
    let connection = &mut db::get(pool).await?;

    // Fetch List of Users and Return
    let users = User::all(connection).await?;

    Ok(Json(users))
}

#[post("/signup", format = "application/json", data = "<user>")]
pub async fn signup(
    _r: RateLimit<'_>,
    pool: &State<DB>,
    redis: &State<Redis>,
    mailer: &State<Email>,
    config: &State<Config>,
    ip: &ClientRealAddr,
    user: Json<NewUser>,
) -> Result<Json<ProteinUser>, Error> {
    // Generate New User Password
    let password_hash = password::generate(&config.password_salt, user.password.clone())?;

    // Validate User
    match user.clone().into_inner().validate() {
        Ok(_) => (),
        Err(error) => return Err(Error::Validation(error.to_string())),
    }

    // Get IP Address
    let ip_address: String = email::get_ip_address(ip)?;

    // Create New User Struct
    let new_user = NewUser {
        username: user.username.clone(),
        password: password_hash,
        email: user.email.clone(),
        last_login_ip: Some(ip_address.clone()),
        ip_address: Some(ip_address.clone()),
    };

    // Create Database Connection
    let connection = &mut db::get(pool).await?;

    // Create User and Grab Result
    let result = User::create(connection, new_user).await?;

    // Generate API Key
    let api_key = APIKey::generate(&result, connection).await?;

    // Set New User in Cache
    Cache::set(redis, "user", result.user_id, Cache::serialize(&result)?).await?;

    // Set API Key In Cache
    Cache::set(
        redis,
        "api_key",
        api_key.api_key,
        Cache::serialize(&api_key)?,
    )
    .await?;

    let verification_link = format!(
        "{}/users/email/verify?token={}",
        config.host_url, result.email_verification_token
    );

    let receipent = result.clone();
    let mail = mailer.inner().clone();
    let smtp = config.inner().clone();

    rocket::tokio::task::spawn(async move {
        let body = Signup {
            name: &receipent.username,
            verification_link: &verification_link,
        };

        match email::send(
            &mail,
            &smtp,
            &receipent,
            "[Security] Account Verification",
            body.render().unwrap(),
            false,
        )
        .await
        {
            Ok(_) => tracing::info!("[Email] ✅ Sent Verification Email"),
            Err(e) => tracing::info!("[Email] ❌ Failed Sending Verification Email: {}", e),
        }
    });

    Ok(Json(ProteinUser {
        user: result,
        api_key: api_key.api_key,
    }))
}

#[get("/email/verify?<token>")]
pub async fn verify_email(
    _r: RateLimit<'_>,
    pool: &State<DB>,
    redis: &State<Redis>,
    mailer: &State<Email>,
    config: &State<Config>,
    token: Uuid,
) -> Result<(ContentType, String), Error> {
    // Create Database Connection
    let connection = &mut db::get(pool).await?;

    // Find User By Token
    let user = User::find_by_email_verification_token(token, connection).await?;

    if !user.email_verified {
        let verified_user = User::verify_email(user.user_id, connection).await?;

        Cache::set(
            redis,
            "user",
            verified_user.user_id,
            Cache::serialize(&verified_user)?,
        )
        .await?;

        let receipent = verified_user.clone();
        let mail = mailer.inner().clone();
        let smtp = config.inner().clone();

        rocket::tokio::task::spawn(async move {
            let body = EmailVerified {
                name: &receipent.username,
            };

            match email::send(
                &mail,
                &smtp,
                &receipent,
                "[Security] Email Successfully Verified",
                body.render().unwrap(),
                false,
            )
            .await
            {
                Ok(_) => tracing::info!("[Email] ✅ Sent Verification Success Email"),
                Err(e) => tracing::info!(
                    "[Email] ❌ Failed Sending Verification Success Email: {}",
                    e
                ),
            }
        });

        let success = Success {
            name: &verified_user.username,
        };

        Ok((ContentType::HTML, success.render().unwrap()))
    } else {
        Err(Error::Email("Email Already Verified".to_string()))
    }
}

#[post("/login", format = "application/json", data = "<user>")]
pub async fn login(
    _r: RateLimit<'_>,
    pool: &State<DB>,
    redis: &State<Redis>,
    config: &State<Config>,
    os: OS<'_>,
    ip: &ClientRealAddr,
    mailer: &State<Email>,
    user: Json<LoginUser>,
) -> Result<Json<Value>, Error> {
    // Create Database Connection
    let connection = &mut db::get(pool).await?;

    // Get Current IP Address
    let ip_address: String = email::get_ip_address(ip)?;

    // Get Current User Agent OS
    let device = email::get_user_agent(&os);

    // Find User
    let result = User::find_by_username(user.username.clone(), connection).await?;

    // Verify Password
    let password_matches = password::verify(result.password.clone(), user.password.clone())?;

    // If Password Entered == Stored Hashed Password
    if password_matches {
        // Grab Current API Key
        let api_key = match APIKey::get_current(&result, connection).await {
            Ok(key) => key,
            Err(_) => {
                tracing::info!(
                    "[+] 🔑 No Valid API Key Found For User: {}, Generating...",
                    result.username
                );

                let new = APIKey::generate(&result, connection).await?;

                // Set API Key in Cache
                Cache::set(redis, "api_key", new.api_key, Cache::serialize(&new)?).await?;

                return Ok(Json(json!({
                    "status": 401,
                    "message": "Your API Key Has Expired. A New Key Has Been Generated. Please Login Again.",
                })));
            }
        };

        // Set User in Cache
        Cache::set(redis, "user", result.user_id, Cache::serialize(&result)?).await?;

        // Set API Key in Cache
        Cache::set(
            redis,
            "api_key",
            api_key.api_key,
            Cache::serialize(&api_key)?,
        )
        .await?;

        // Send New Login If IP Address != Original
        if result.last_login_ip != ip_address {
            let receipent = result.clone();
            let mail = mailer.inner().clone();
            let smtp = config.inner().clone();
            let now = chrono::Utc::now().naive_utc();

            // Update User With New IP Address
            let data = User::update_ip_info(result.user_id, &ip_address, &now, connection).await?;

            // Update Cache
            Cache::set(redis, "user", data.user_id, Cache::serialize(&data)?).await?;

            // Send Email
            rocket::tokio::task::spawn(async move {
                let body = Login {
                    name: &receipent.username,
                    time: &email::format_date(&now),
                    ip_address: &ip_address,
                    device: &device,
                };

                match email::send(
                    &mail,
                    &smtp,
                    &receipent,
                    "[Security] New Login",
                    body.render().unwrap(),
                    false,
                )
                .await
                {
                    Ok(_) => tracing::info!("[Email] ✅ Sent New Login Email"),
                    Err(e) => tracing::info!("[Email] ❌ Failed Sending New Login Email: {}", e),
                }
            });
        }

        // MFA
        if result.mfa_enabled {
            // Generate MFA Code and Expiry
            // MFA Codes Are 6 Digits: 12346
            // Expires By Default In 10 Minutes
            let mfa_user = User::generate_mfa_code(result.user_id, connection).await?;

            // Update Cache
            Cache::set(
                redis,
                "user",
                mfa_user.user_id,
                Cache::serialize(&mfa_user)?,
            )
            .await?;

            // Get Successfully Generated Code
            let code = match mfa_user.mfa_code {
                Some(ref code) => code.clone(),
                None => return Err(Error::Internal("MFA Code Not Generated".to_string())),
            };

            // Send Email
            let mail = mailer.inner().clone();
            let smtp = config.inner().clone();

            rocket::tokio::task::spawn(async move {
                let body = MFACode {
                    name: &mfa_user.username,
                    code: &code,
                };

                match email::send(
                    &mail,
                    &smtp,
                    &mfa_user,
                    "[Security] MFA Code",
                    body.render().unwrap(),
                    false,
                )
                .await
                {
                    Ok(_) => tracing::info!("[Email] ✅ Sent MFA Code Email"),
                    Err(e) => tracing::info!("[Email] ❌ Failed Sending MFA Code Email: {}", e),
                }
            });

            // Return MFA Code Required
            return Ok(Json(json!({
                "status": 200,
                "user_id": result.user_id,
                "message": "MFA Code Required & Sent",
            })));
        }

        Ok(Json(json!({
            "status": 200,
            "message": "Successful Login!",
            "user": result,
            "user_id": result.user_id,
            "api_key": api_key.api_key,
        })))
    } else {
        Err(Error::Authorization(
            "Invalid Username or Password".to_string(),
        ))
    }
}

#[get("/delete?<user_id>", format = "application/json")]
pub async fn delete_user(
    _r: RateLimit<'_>,
    _auth: API,
    mailer: &State<Email>,
    pool: &State<DB>,
    redis: &State<Redis>,
    config: &State<Config>,
    user_id: Uuid,
) -> Result<status::Accepted<Value>, Error> {
    // Create Database Connection
    let connection = &mut db::get(pool).await?;

    // Delete User
    let user = User::delete(user_id, connection).await?;

    Cache::delete(redis, "user", user_id).await?;

    // send account deletion email here
    let receipent = user.clone();
    let mail = mailer.inner().clone();
    let smtp = config.inner().clone();

    rocket::tokio::task::spawn(async move {
        let body = AccountDeleted {
            name: &receipent.username,
        };

        match email::send(
            &mail,
            &smtp,
            &receipent,
            "[Security] Your Account Has Been Deleted",
            body.render().unwrap(),
            false,
        )
        .await
        {
            Ok(_) => tracing::info!("[Email] ✅ Sent Account Deletion Email"),
            Err(e) => tracing::info!("[Email] ❌ Failed Sending Account Deletion Email: {}", e),
        }
    });

    // Return Okay Message with Deleted ID
    Ok(status::Accepted(json!({
        "status": 200,
        "message": "User Deleted Successfully",
        "user": user,
    })))
}

#[post("/update?<user_id>", format = "application/json", data = "<user>")]
pub async fn update_user(
    _r: RateLimit<'_>,
    _auth: API,
    user_id: Uuid,
    pool: &State<DB>,
    discord: &State<Webhook>,
    redis: &State<Redis>,
    user: Json<UpdateUser>,
) -> Result<Json<User>, Error> {
    // Create Database Connection
    let connection = &mut db::get(pool).await?;

    // Update User
    let updated_user = User::update(user_id, user.into_inner(), connection).await?;

    // Update Cache With User
    Cache::set(redis, "user", user_id, Cache::serialize(&updated_user)?).await?;

    // Send Webhook
    let discord_user = updated_user.clone();
    let dis = discord.inner().clone();

    rocket::tokio::task::spawn(async move {
        webhook::send(
            &dis,
            "User Has Been Updated!",
            discord_user.username.as_str(),
            discord_user.last_login_ip.as_str(),
            "[User]",
        )
        .await
    });

    // Return Updated User
    Ok(Json(updated_user))
}

#[post(
    "/update/password?<user_id>",
    format = "application/json",
    data = "<data>"
)]
pub async fn update_user_password(
    _r: RateLimit<'_>,
    _auth: API,
    user_id: Uuid,
    ip: &ClientRealAddr,
    mailer: &State<Email>,
    pool: &State<DB>,
    redis: &State<Redis>,
    config: &State<Config>,
    data: Json<Password>,
) -> Result<Json<User>, Error> {
    // Create Database Connection
    let connection = &mut db::get(pool).await?;

    // Get IP Address
    let ip_address: String = email::get_ip_address(ip)?;

    // Grab User
    let user = User::find(user_id, connection).await?;

    // Verify Old Password
    let old_password_hash = password::verify(user.password.clone(), data.old_password.clone())?;

    // False = Wrong Password
    if !old_password_hash {
        return Err(Error::Authorization("Invalid Password".to_string()));
    }

    // Generate New Password Hash
    let password_hash = password::generate(&config.password_salt, data.new_password.clone())?;

    // Update Password
    let updated_user = User::update_password(user_id, &password_hash, connection).await?;

    // Update Cache With User
    Cache::set(redis, "user", user_id, Cache::serialize(&updated_user)?).await?;

    // Send Email
    let receipent = updated_user.clone();
    let mail = mailer.inner().clone();
    let smtp = config.inner().clone();

    rocket::tokio::task::spawn(async move {
        let body = UpdatedPassword {
            name: &receipent.username,
            ip_address: &ip_address,
        };

        match email::send(
            &mail,
            &smtp,
            &receipent,
            "[Security] Your Password Has Been Updated",
            body.render().unwrap(),
            false,
        )
        .await
        {
            Ok(_) => tracing::info!("[Email] ✅ Sent Password Update Email"),
            Err(e) => tracing::info!("[Email] ❌ Failed Sending Password Update Email: {}", e),
        }
    });

    Ok(Json(updated_user))
}

#[get("/auth/request-password-reset?<email>", format = "application/json")]
pub async fn request_password_reset(
    _r: RateLimit<'_>,
    mailer: &State<Email>,
    pool: &State<DB>,
    redis: &State<Redis>,
    config: &State<Config>,
    email: String,
) -> Result<status::Accepted<Value>, Error> {
    let connection = &mut db::get(pool).await?;

    let user = User::find_by_email(email, connection).await?;

    if user.mfa_enabled {
        // Generate MFA Code and Expiry
        // MFA Codes Are 6 Digits: 12346
        // Expires By Default In 10 Minutes
        let mfa_user = User::generate_mfa_code(user.user_id, connection).await?;

        // Update Cache
        Cache::set(
            redis,
            "user",
            mfa_user.user_id,
            Cache::serialize(&mfa_user)?,
        )
        .await?;

        // Get Successfully Generated Code
        let code = match mfa_user.mfa_code {
            Some(ref code) => code.clone(),
            None => return Err(Error::Internal("MFA Code Not Generated".to_string())),
        };

        // Send Email
        let mail = mailer.inner().clone();
        let smtp = config.inner().clone();

        rocket::tokio::task::spawn(async move {
            let body = RequestPasswordReset {
                username: &mfa_user.username,
                code: &code,
            };

            match email::send(
                &mail,
                &smtp,
                &mfa_user,
                "[Security] MFA Code",
                body.render().unwrap(),
                false,
            )
            .await
            {
                Ok(_) => tracing::info!("[Email] ✅ Sent MFA Code Email"),
                Err(e) => tracing::info!("[Email] ❌ Failed Sending MFA Code Email: {}", e),
            }
        });

        Ok(status::Accepted(json!({
            "status": 200,
            "message": "MFA Code Sent",
            "user_id": user.user_id,
        })))
    } else {
        return Err(Error::Authorization(
            "MFA Not Enabled, Cannot Reset Password".to_string(),
        ));
    }
}

#[post("/auth/reset-password", format = "application/json", data = "<data>")]
pub async fn reset_password(
    _r: RateLimit<'_>,
    mailer: &State<Email>,
    pool: &State<DB>,
    redis: &State<Redis>,
    config: &State<Config>,
    os: OS<'_>,
    ip: &ClientRealAddr,
    data: Json<ForgotPassword>,
) -> Result<status::Accepted<Value>, Error> {
    // Grab Database Connection
    let connection = &mut db::get(pool).await?;

    // Get Current IP Address
    let ip_address: String = email::get_ip_address(&ip)?;

    // Get Current User Agent OS
    let device = email::get_user_agent(&os);

    // Get User Profile
    let user = User::find_by_email(data.email.clone(), connection).await?;

    if let Some(_code) = user.mfa_code {
        return Err(Error::Authorization(
            "MFA Code Found, Please Verify To Reset".to_string(),
        ));
    }

    // Hash Generated Password
    let hashed_password = password::generate(&config.password_salt, data.password.clone())?;

    // Update User Password With New Hashed Data
    let updated_user = User::update_password(user.user_id, &hashed_password, connection).await?;

    // Update Cache
    Cache::set(
        redis,
        "user",
        updated_user.user_id,
        Cache::serialize(&updated_user)?,
    )
    .await?;

    // Send Email
    let receipent = user.clone();
    let mail = mailer.inner().clone();
    let smtp = config.inner().clone();

    rocket::tokio::task::spawn(async move {
        let body = PasswordReset {
            name: &receipent.username,
            new_password: &data.password,
            ip_address: &ip_address,
            device: &device,
        };

        match email::send(
            &mail,
            &smtp,
            &receipent,
            "[Security] Your Password Has Been Reset",
            body.render().unwrap(),
            false,
        )
        .await
        {
            Ok(_) => tracing::info!("[Email] ✅ Sent Password Reset Email"),
            Err(e) => tracing::info!("[Email] ❌ Failed Sending Password Reset Email: {}", e),
        }
    });

    Ok(status::Accepted(json!({
        "status": 200,
        "message": "Forgot Password Email Sent!",
        "user_id": user.user_id,
    })))
}

#[get("/mfa/resend?<user_id>", format = "application/json")]
pub async fn resend_mfa(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
    mailer: &State<Email>,
    config: &State<Config>,
    user_id: Uuid,
) -> Result<Json<Value>, Error> {
    let connection = &mut db::get(pool).await?;

    let user = User::find(user_id, connection).await?;

    if !user.mfa_enabled {
        return Err(Error::Authorization("MFA Not Enabled".to_string()));
    }

    if !user.mfa_verified {
        return Err(Error::Authorization("MFA Not Verified".to_string()));
    }

    match user.mfa_code_expires_at {
        Some(expired) => {
            let now: i64 = chrono::Utc::now().naive_utc().and_utc().timestamp();

            if now > expired.and_utc().timestamp() {
                return Err(Error::Authorization(
                    "MFA Code Expired, Login Again".to_string(),
                ));
            }
        }
        None => {
            return Err(Error::Authorization(
                "MFA Code Potentially Expired".to_string(),
            ));
        }
    }

    let mfa_user = user.clone();

    match user.mfa_code {
        Some(code) => {
            // Send Email
            let mail = mailer.inner().clone();
            let smtp = config.inner().clone();

            rocket::tokio::task::spawn(async move {
                let body = MFACode {
                    name: &user.username,
                    code: &code,
                };

                match email::send(
                    &mail,
                    &smtp,
                    &mfa_user,
                    "[Security] MFA Code",
                    body.render().unwrap(),
                    false,
                )
                .await
                {
                    Ok(_) => tracing::info!("[Email] ✅ Sent MFA Code Email"),
                    Err(e) => tracing::info!("[Email] ❌ Failed Sending MFA Code Email: {}", e),
                }
            });

            // Return MFA Code Required
            return Ok(Json(json!({
                "status": 200,
                "user_id": user_id,
                "message": "MFA Code Resent",
            })));
        }
        None => return Err(Error::Authorization("No MFA Code Found".to_string())),
    }
}

#[get("/mfa/enable?<user_id>", format = "application/json")]
pub async fn enable_mfa(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
    redis: &State<Redis>,
    mailer: &State<Email>,
    config: &State<Config>,
    user_id: Uuid,
) -> Result<Json<User>, Error> {
    let connection = &mut db::get(pool).await?;

    let user = User::find(user_id, connection).await?;

    if user.mfa_enabled {
        return Err(Error::Authorization("MFA Already Enabled".to_string()));
    }

    let mfa_user = User::enable_mfa(user_id, connection).await?;

    // Update Cache
    Cache::set(redis, "user", user_id, Cache::serialize(&mfa_user)?).await?;

    // Send MFA Verification Link
    let verification_link = format!(
        "{}/users/mfa/verify/?token={}",
        config.host_url, mfa_user.mfa_verification_token
    );

    let receipent = mfa_user.clone();
    let mail = mailer.inner().clone();
    let smtp = config.inner().clone();

    rocket::tokio::task::spawn(async move {
        let body = MFAVerification {
            name: &receipent.username,
            verification_link: &verification_link,
        };

        match email::send(
            &mail,
            &smtp,
            &receipent,
            "[Security] Account MFA Verification",
            body.render().unwrap(),
            false,
        )
        .await
        {
            Ok(_) => tracing::info!("[Email] ✅ Sent MFA Verification Email"),
            Err(e) => tracing::info!("[Email] ❌ Failed Sending MFA Verification Email: {}", e),
        }
    });

    Ok(Json(mfa_user))
}

#[get("/mfa/request/<code>?<user_id>", format = "application/json")]
pub async fn password_request_check_code(
    _r: RateLimit<'_>,
    pool: &State<DB>,
    redis: &State<Redis>,
    user_id: Uuid,
    code: &str,
) -> Result<Json<Value>, Error> {
    let connection = &mut db::get(pool).await?;

    let user = User::find(user_id, connection).await?;

    let key = match APIKey::get_current(&user, connection).await {
        Ok(key) => key,
        Err(_) => {
            tracing::info!(
                "[+] 🔑 No Valid API Key Found For User: {}, Generating...",
                user.username
            );

            let new = APIKey::generate(&user, connection).await?;

            // Set API Key in Cache
            Cache::set(redis, "api_key", new.api_key, Cache::serialize(&new)?).await?;

            return Ok(Json(json!({
                "status": 401,
                "message": "Your API Key Has Expired. A New Key Has Been Generated. Please Login Again.",
            })));
        }
    };

    if user.mfa_enabled {
        if !user.mfa_verified {
            return Err(Error::Authorization("MFA Not Verified".to_string()));
        }

        let now: i64 = chrono::Utc::now().naive_utc().and_utc().timestamp();

        if let Some(expired) = user.mfa_code_expires_at {
            if now > expired.and_utc().timestamp() {
                User::reset_mfa(user_id, connection).await?;

                return Err(Error::Authorization(
                    "MFA Code Expired, Login Again".to_string(),
                ));
            }
        } else {
            return Err(Error::Authorization(
                "MFA Code Potentially Expired".to_string(),
            ));
        }

        if let Some(ref mfa_code) = user.mfa_code {
            if *mfa_code == code {
                let result = User::reset_mfa(user_id, connection).await?;

                // Update Cache
                Cache::set(redis, "user", user_id, Cache::serialize(&result)?).await?;

                return Ok(Json(json!(
                    {
                        "status": 200,
                        "message": "MFA Code Verified",
                        "user": result,
                        "api_key": key.api_key
                    }
                )));
            } else {
                return Err(Error::Authorization("Invalid MFA Code".to_string()));
            }
        } else {
            return Err(Error::Authorization("No MFA Code".to_string()));
        }
    } else {
        return Err(Error::Authorization("MFA Not Enabled".to_string()));
    }
}

#[get("/mfa/check/<code>?<user_id>", format = "application/json")]
pub async fn check_mfa(
    _r: RateLimit<'_>,
    pool: &State<DB>,
    redis: &State<Redis>,
    mailer: &State<Email>,
    config: &State<Config>,
    ip: &ClientRealAddr,
    os: OS<'_>,
    user_id: Uuid,
    code: &str,
) -> Result<Json<Value>, Error> {
    let connection = &mut db::get(pool).await?;

    let ip_address: String = email::get_ip_address(ip)?;

    // Get Current User Agent OS
    let device = email::get_user_agent(&os);

    let user = User::find(user_id, connection).await?;

    let key = match APIKey::get_current(&user, connection).await {
        Ok(key) => key,
        Err(_) => {
            tracing::info!(
                "[+] 🔑 No Valid API Key Found For User: {}, Generating...",
                user.username
            );

            let new = APIKey::generate(&user, connection).await?;

            // Set API Key in Cache
            Cache::set(redis, "api_key", new.api_key, Cache::serialize(&new)?).await?;

            return Ok(Json(json!({
                "status": 401,
                "message": "Your API Key Has Expired. A New Key Has Been Generated. Please Login Again.",
            })));
        }
    };

    if user.mfa_enabled {
        if !user.mfa_verified {
            return Err(Error::Authorization("MFA Not Verified".to_string()));
        }

        let now: i64 = chrono::Utc::now().naive_utc().and_utc().timestamp();

        if let Some(expired) = user.mfa_code_expires_at {
            if now > expired.and_utc().timestamp() {
                User::reset_mfa(user_id, connection).await?;

                return Err(Error::Authorization(
                    "MFA Code Expired, Login Again".to_string(),
                ));
            }
        } else {
            return Err(Error::Authorization(
                "MFA Code Potentially Expired".to_string(),
            ));
        }

        if let Some(ref mfa_code) = user.mfa_code {
            if *mfa_code == code {
                let result = User::reset_mfa(user_id, connection).await?;

                // Update Cache
                Cache::set(redis, "user", user_id, Cache::serialize(&result)?).await?;

                if result.last_login_ip != ip_address {
                    let receipent = result.clone();
                    let mail = mailer.inner().clone();
                    let smtp = config.inner().clone();
                    let now = chrono::Utc::now().naive_utc();

                    rocket::tokio::task::spawn(async move {
                        let body = Login {
                            name: &receipent.username,
                            time: &email::format_date(&now),
                            ip_address: &ip_address,
                            device: &device,
                        };

                        match email::send(
                            &mail,
                            &smtp,
                            &receipent,
                            "[Security] New Login",
                            body.render().unwrap(),
                            false,
                        )
                        .await
                        {
                            Ok(_) => tracing::info!("[Email] ✅ Sent New Login Email"),
                            Err(e) => {
                                tracing::info!("[Email] ❌ Failed Sending New Login Email: {}", e)
                            }
                        }
                    });
                }

                return Ok(Json(json!(
                    {
                        "status": 200,
                        "message": "MFA Code Verified",
                        "user": result,
                        "api_key": key.api_key
                    }
                )));
            } else {
                return Err(Error::Authorization("Invalid MFA Code".to_string()));
            }
        } else {
            return Err(Error::Authorization("No MFA Code".to_string()));
        }
    } else {
        return Err(Error::Authorization("MFA Not Enabled".to_string()));
    }
}

#[get("/mfa/verify?<token>")]
pub async fn verify_mfa(
    _r: RateLimit<'_>,
    pool: &State<DB>,
    redis: &State<Redis>,
    token: Uuid,
) -> Result<(ContentType, String), Error> {
    let connection = &mut db::get(pool).await?;

    // Find User By Token
    let user = User::find_by_mfa_verification_token(token, connection).await?;

    if !user.mfa_verified && user.mfa_enabled {
        let verified_user = User::verify_mfa(user.user_id, connection).await?;

        Cache::set(
            redis,
            "user",
            verified_user.user_id,
            Cache::serialize(&verified_user)?,
        )
        .await?;

        let success = Success {
            name: &verified_user.username,
        };

        Ok((ContentType::HTML, success.render().unwrap()))
    } else {
        Err(Error::Email("MFA Already Verified".to_string()))
    }
}

#[get("/mfa/disable?<user_id>", format = "application/json")]
pub async fn disable_mfa(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
    redis: &State<Redis>,
    mailer: &State<Email>,
    config: &State<Config>,
    user_id: Uuid,
) -> Result<Json<User>, Error> {
    let connection = &mut db::get(pool).await?;

    let user = User::disable_mfa(user_id, connection).await?;

    Cache::set(redis, "user", user_id, Cache::serialize(&user)?).await?;

    let receipent = user.clone();
    let mail = mailer.inner().clone();
    let smtp = config.inner().clone();

    rocket::tokio::task::spawn(async move {
        let body = MFADisabled {
            name: &receipent.username,
        };

        match email::send(
            &mail,
            &smtp,
            &receipent,
            "[Security] MFA Disabled",
            body.render().unwrap(),
            false,
        )
        .await
        {
            Ok(_) => tracing::info!("[Email] ✅ Sent MFA Notification Email"),
            Err(e) => tracing::info!("[Email] ❌ Failed Sending MFA Notification Email: {}", e),
        }
    });

    Ok(Json(user))
}

#[get("/auth/email?<email>", format = "application/json")]
pub async fn get_user_by_email(
    _r: RateLimit<'_>,
    pool: &State<DB>,
    email: String,
) -> Result<Json<User>, Error> {
    let connection = &mut db::get(pool).await?;

    let user = User::find_by_email(email, connection).await?;

    Ok(Json(user))
}

#[get("/auth/elevate/<user>?<user_id>", format = "application/json")]
pub async fn elevate_user(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
    redis: &State<Redis>,
    user: Uuid,
    user_id: Uuid,
) -> Result<Json<User>, Error> {
    let connection = &mut db::get(pool).await?;

    let result = User::elevate(user, connection).await?;

    Cache::set(redis, "user", result.user_id, Cache::serialize(&result)?).await?;

    Ok(Json(result))
}
