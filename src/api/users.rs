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
    State, get, post,
    response::status,
    serde::{
        json::{Json, Value, json},
        uuid::Uuid,
    },
};
// Validation
use validator::Validate;

// Protein
use crate::{
    auth::api::API,
    auth::rate_limit::RateLimit,
    cache::redis::{Cache, RedisPool},
    db,
    db::DatabasePool,
    errors::ProteinError,
    models::keys::APIKey,
    models::user::{ForgotPassword, LoginUser, NewUser, Password, ProteinUser, UpdateUser, User},
    utils::email,
    utils::email::Mailer,
    utils::password,
    utils::webhook,
    utils::webhook::Webhook,
};

#[get("/?<user_id>", format = "application/json")]
pub async fn get_user(
    _r: RateLimit<'_>,
    _auth: API,
    user_id: Uuid,
    pool: &State<DatabasePool>,
    redis: &State<RedisPool>,
) -> Result<Json<User>, ProteinError> {
    // Check Cache
    let cache: Value = Cache::get(redis, "user", user_id.to_string()).await?;

    // If Cache is Null, Fetch From Database
    if cache.is_null() {
        // Crate Database Connection
        let connection = &mut db::get_connection(pool).await?;

        // Grab User
        let user = User::find(user_id, connection).await?;

        // Set User in Cache
        Cache::set(redis, "user", user_id.to_string(), Cache::serialize(&user)?).await?;

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
    pool: &State<DatabasePool>,
) -> Result<Json<Vec<User>>, ProteinError> {
    // Creating Database Connection
    let connection = &mut db::get_connection(pool).await?;

    // Fetch List of Users and Return
    let users = User::all(connection).await?;

    Ok(Json(users))
}

#[post("/signup", format = "application/json", data = "<user>")]
pub async fn signup(
    _r: RateLimit<'_>,
    pool: &State<DatabasePool>,
    redis: &State<RedisPool>,
    mailer: &State<Mailer>,
    ip: &ClientRealAddr,
    user: Json<NewUser>,
) -> Result<Json<ProteinUser>, ProteinError> {
    // Generate New User Password
    let password_hash = password::generate_hashed_password(user.password.clone())?;

    // Validate User
    match user.clone().into_inner().validate() {
        Ok(_) => (),
        Err(error) => return Err(ProteinError::Validation(error.to_string())),
    }

    // Get IP Address
    let ip_address: Option<String> = match ip.get_ipv4_string() {
        Some(addr) => Some(addr),
        None => return Err(ProteinError::Internal("Invalid IP Address".to_string())),
    };

    // Create New User Struct
    let new_user = NewUser {
        username: user.username.clone(),
        password: password_hash,
        email: user.email.clone(),
        last_login_ip: ip_address.clone(),
        ip_address: ip_address.clone(),
    };

    // Create Database Connection
    let connection = &mut db::get_connection(pool).await?;

    // Create User and Grab Result
    let result = User::create(connection, new_user).await?;

    // Generate API Key
    let api_key = APIKey::generate(&result, connection).await?;

    // Set New User in Cache
    Cache::set(
        redis,
        "user",
        result.user_id.to_string(),
        Cache::serialize(&result)?,
    )
    .await?;

    // Set API Key In Cache
    Cache::set(
        redis,
        "api_key",
        api_key.api_key.to_string(),
        Cache::serialize(&api_key)?,
    )
    .await?;

    // Send Email Verification Link
    let host = std::env::var("HOST_URL").unwrap_or_else(|_| "http://localhost:8000/v1".to_string());

    let verification_link = format!(
        "{}/users/email/verify?token={}",
        host, result.email_verification_token
    );

    let receipent = result.clone();
    let mail = mailer.inner().clone();

    rocket::tokio::task::spawn(async move {
        match email::send_email(&mail, &receipent, "[Security] Account Verification", format!("Hello {}! \n\nWelcome To Hypertrophy!\n\nWe Hope You Enjoy The Multitude of Features Provided\n\nPlease Verify Your Email At This Link:\n\n\n{}", receipent.username, verification_link)).await {
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
    pool: &State<DatabasePool>,
    mailer: &State<Mailer>,
    token: Uuid,
) -> Result<Json<Value>, ProteinError> {
    // Create Database Connection
    let connection = &mut db::get_connection(pool).await?;

    // Find User By Token
    let user = User::find_by_email_verification_token(token, connection).await?;

    if !user.email_verified {
        let verified_user = User::verify_email(user.user_id, connection).await?;

        let receipent = verified_user.clone();
        let mail = mailer.inner().clone();

        rocket::tokio::task::spawn(async move {
            match email::send_email(
                &mail,
                &receipent,
                "[Security] Email Successfully Verified",
                format!(
                    "Hello {}! \n\nYour Email Has Been Successfully Verified!",
                    receipent.username
                ),
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

        Ok(Json(json!({
            "message": "Email Verified Successfully",
            "user": verified_user,
        })))
    } else {
        Err(ProteinError::Email("Email Already Verified".to_string()))
    }
}

#[post("/login", format = "application/json", data = "<user>")]
pub async fn login(
    _r: RateLimit<'_>,
    pool: &State<DatabasePool>,
    redis: &State<RedisPool>,
    os: OS<'_>,
    ip: &ClientRealAddr,
    mailer: &State<Mailer>,
    user: Json<LoginUser>,
) -> Result<Json<Value>, ProteinError> {
    // Create Database Connection
    let connection = &mut db::get_connection(pool).await?;

    // Get Current IP Address
    let ip_address: String = match ip.get_ipv4_string() {
        Some(addr) => addr,
        None => String::from("unknown"),
    };

    // Get Current User Agent OS
    let device = format!(
        "{} {}",
        os.name.unwrap_or_else(|| "_".to_owned().into()),
        os.major.unwrap_or_else(|| "_".to_owned().into())
    );

    // Find User
    let result = User::find_by_username(user.username.clone(), connection).await?;

    // Verify Password
    let password_matches =
        password::verify_password(result.password.clone(), user.password.clone())?;

    if password_matches {
        let api_key = APIKey::get(&result, connection).await?;

        // Set User in Cache
        Cache::set(
            redis,
            "user",
            result.user_id.to_string(),
            Cache::serialize(&result)?,
        )
        .await?;

        // Set API Key in Cache
        Cache::set(
            redis,
            "api_key",
            api_key.api_key.to_string(),
            Cache::serialize(&api_key)?,
        )
        .await?;

        // send new login email here
        if result.last_login_ip != ip_address {
            let receipent = result.clone();
            let mail = mailer.inner().clone();

            // Update User With New IP Address
            User::update_ip_info(
                result.user_id,
                &ip_address,
                chrono::Utc::now().naive_utc(),
                connection,
            )
            .await?;

            rocket::tokio::task::spawn(async move {
                match email::send_email(&mail, &receipent, "[Security] New Login", format!("Hello {}! \n\nThis Email Serves As Confirmation That There Has Been A New Login Into Your Account.\n\n\nIf You Did NOT Request This, Please Ignore This Email\nRequest IP Address: {}\nDevice: {}", receipent.username, ip_address, device)).await {
                    Ok(_) => tracing::info!("[Email] ✅ Sent New Login Email"),
                    Err(e) => tracing::info!("[Email] ❌ Failed Sending New Login Email: {}", e),
                }
            });
        }

        // MFA
        if result.mfa_enabled {
            // generate mfa code && mfa code expires at
            let mfa_user = User::generate_mfa_code(result.user_id, connection).await?;

            let code = match mfa_user.mfa_code {
                Some(ref code) => code.clone(),
                None => return Err(ProteinError::Internal("MFA Code Not Generated".to_string())),
            };

            // send mfa code email
            let mail = mailer.inner().clone();

            rocket::tokio::task::spawn(async move {
                match email::send_email(&mail, &mfa_user, "[Security] MFA Code", format!("Hello {}! \n\nThis Email Serves As Your MFA Code!\n\n\n{}\n\nCode Expires In 10 Minutes!", mfa_user.username, code)).await {
                    Ok(_) => tracing::info!("[Email] ✅ Sent MFA Code Email"),
                    Err(e) => tracing::info!("[Email] ❌ Failed Sending MFA Code Email: {}", e),
                }
            });

            // return mfa code required
            return Ok(Json(json!({
                "message": "MFA Code Required & Sent",
            })));
        }

        Ok(Json(json!({
            "message": "Successful Login!",
            "user": result,
            "api_key": api_key.api_key,
        })))
    } else {
        Err(ProteinError::Authorization(
            "Invalid Username or Password".to_string(),
        ))
    }
}

#[get("/delete?<user_id>", format = "application/json")]
pub async fn delete_user(
    _r: RateLimit<'_>,
    _auth: API,
    user_id: Uuid,
    mailer: &State<Mailer>,
    pool: &State<DatabasePool>,
) -> Result<status::Accepted<Value>, ProteinError> {
    // Create Database Connection
    let connection = &mut db::get_connection(pool).await?;

    // Delete User
    let user = User::delete(user_id, connection).await?;

    // send account deletion email here
    let receipent = user.clone();
    let mail = mailer.inner().clone();

    rocket::tokio::task::spawn(async move {
        match email::send_email(&mail, &receipent, "[Security] Your Account Has Been Deleted", format!("Hello {}! \n\nThis Email Serves As Confirmation That Your Account Has Been Successfully Deleted.\n\n\nIf You Did NOT Request This, Please Contact Support Immediately", receipent.username)).await {
            Ok(_) => tracing::info!("[Email] ✅ Sent Account Deletion Email"),
            Err(e) => tracing::info!("[Email] ❌ Failed Sending Account Deletion Email: {}", e),
        }
    });

    // Return Okay Message with Deleted ID
    Ok(status::Accepted(json!({
        "message": "User Deleted Successfully",
        "user": user,
    })))
}

#[post("/update?<user_id>", format = "application/json", data = "<user>")]
pub async fn update_user(
    _r: RateLimit<'_>,
    _auth: API,
    user_id: Uuid,
    pool: &State<DatabasePool>,
    discord: &State<Webhook>,
    redis: &State<RedisPool>,
    user: Json<UpdateUser>,
) -> Result<Json<User>, ProteinError> {
    // Create Database Connection
    let connection = &mut db::get_connection(pool).await?;

    // Update User
    let updated_user = User::update(user_id, user.into_inner(), connection).await?;

    // Update Cache With User
    Cache::set(
        redis,
        "user",
        user_id.to_string(),
        Cache::serialize(&updated_user)?,
    )
    .await?;

    // Send Webhook
    let discord_user = updated_user.clone();
    let dis = discord.inner().clone();

    rocket::tokio::task::spawn(async move {
        webhook::send_audit_log(
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
    mailer: &State<Mailer>,
    pool: &State<DatabasePool>,
    redis: &State<RedisPool>,
    data: Json<Password>,
) -> Result<Json<User>, ProteinError> {
    // Create Database Connection
    let connection = &mut db::get_connection(pool).await?;

    // Get IP Address
    let ip_address: String = match ip.get_ipv4_string() {
        Some(addr) => addr,
        None => String::from("unknown"),
    };

    // Grab User
    let user = User::find(user_id, connection).await?;

    // Verify Old Password
    let old_password_hash =
        password::verify_password(user.password.clone(), data.old_password.clone())?;

    // False = Wrong Password
    if !old_password_hash {
        return Err(ProteinError::Authorization("Invalid Password".to_string()));
    }

    // Generate New Password Hash
    let password_hash = password::generate_hashed_password(data.new_password.clone())?;

    // Update Password
    let updated_user = User::update_password(user_id, &password_hash, connection).await?;

    // Update Cache With User
    Cache::set(
        redis,
        "user",
        user_id.to_string(),
        Cache::serialize(&updated_user)?,
    )
    .await?;

    // send email
    let receipent = updated_user.clone();
    let mail = mailer.inner().clone();

    rocket::tokio::task::spawn(async move {
        match email::send_email(&mail, &receipent, "[Security] Your Password Has Been Updated", format!("Hello {}! \n\nThis Email Serves As Confirmation That Your Password Has Been Successfully Updated.\n\n\nIf You Did NOT Request This, Please Contact Support Immediately\n\nIP Address: {}", receipent.username, ip_address)).await {
            Ok(_) => tracing::info!("[Email] ✅ Sent Password Update Email"),
            Err(e) => tracing::info!("[Email] ❌ Failed Sending Password Update Email: {}", e),
        }
    });

    Ok(Json(updated_user))
}

#[post("/email/forgot-password", format = "application/json", data = "<data>")]
pub async fn forgot_password_email(
    _r: RateLimit<'_>,
    mailer: &State<Mailer>,
    pool: &State<DatabasePool>,
    os: OS<'_>,
    ip: &ClientRealAddr,
    data: Json<ForgotPassword>,
) -> Result<status::Accepted<Value>, ProteinError> {
    // Grab Database Connection
    let connection = &mut db::get_connection(pool).await?;

    // Get Current IP Address
    let ip_address: String = match ip.get_ipv4_string() {
        Some(addr) => addr,
        None => String::from("unknown"),
    };

    // Get Current User Agent OS
    let device = format!(
        "{} {}",
        os.name.unwrap_or_else(|| "_".to_owned().into()),
        os.major.unwrap_or_else(|| "_".to_owned().into())
    );

    // Get User Profile
    let user = User::find_by_email(data.email.clone(), connection).await?;

    // Hash Generated Password
    let hashed_password = password::generate_hashed_password(data.password.clone())?;

    // Update User Password With New Hashed Data
    User::update_password(user.user_id, &hashed_password, connection).await?;

    // Send Email
    let receipent = user.clone();
    let mail = mailer.inner().clone();

    rocket::tokio::task::spawn(async move {
        match email::send_email(&mail, &receipent, "[Security] Your Password Has Been Reset", format!("Hello {}! \n\nThis Email Serves As Confirmation That Your Password Has Been Successfully Reset.\n\nNew Password: {}\n\n\nIf You Did NOT Request This, Please Ignore This Email\nRequest IP Address: {}\nDevice: {}", receipent.username, data.password, ip_address, device)).await {
            Ok(_) => tracing::info!("[Email] ✅ Sent Password Reset Email"),
            Err(e) => tracing::info!("[Email] ❌ Failed Sending Password Reset Email: {}", e),
        }
    });

    Ok(status::Accepted(json!({
        "message": "Forgot Password Email Sent!",
        "user_id": user.user_id,
    })))
}

#[get("/mfa/enable?<user_id>", format = "application/json")]
pub async fn enable_mfa(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DatabasePool>,
    mailer: &State<Mailer>,
    user_id: Uuid,
) -> Result<Json<User>, ProteinError> {
    let connection = &mut db::get_connection(pool).await?;

    let user = User::find(user_id, connection).await?;

    if user.mfa_enabled {
        return Err(ProteinError::Authorization(
            "MFA Already Enabled".to_string(),
        ));
    }

    let mfa_user = User::enable_mfa(user_id, connection).await?;

    // Send MFA Verification Link
    let host = std::env::var("HOST_URL").unwrap_or_else(|_| "http://localhost:8000/v1".to_string());

    let verification_link = format!(
        "{}/users/mfa/verify/?token={}",
        host, mfa_user.mfa_verification_token
    );

    let receipent = mfa_user.clone();
    let mail = mailer.inner().clone();

    rocket::tokio::task::spawn(async move {
        match email::send_email(&mail, &receipent, "[Security] Account MFA Verification", format!("Hello {}! \n\nThis Email Serves As Verification For Enabling MFA!\nPlease Verify MFA At This Link:\n\n\n{}", receipent.username, verification_link)).await {
            Ok(_) => tracing::info!("[Email] ✅ Sent MFA Verification Email"),
            Err(e) => tracing::info!("[Email] ❌ Failed Sending MFA Verification Email: {}", e),
        }
    });

    Ok(Json(mfa_user))
}

#[get("/mfa/check/<code>?<user_id>", format = "application/json")]
pub async fn check_mfa(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DatabasePool>,
    mailer: &State<Mailer>,
    user_id: Uuid,
    code: &str,
) -> Result<Json<Value>, ProteinError> {
    let connection = &mut db::get_connection(pool).await?;

    let user = User::find(user_id, connection).await?;

    if user.mfa_enabled {
        if !user.mfa_verified {
            return Err(ProteinError::Authorization("MFA Not Verified".to_string()));
        }

        let now: i64 = chrono::Utc::now().naive_utc().and_utc().timestamp();

        if let Some(expired) = user.mfa_code_expires_at {
            if now > expired.and_utc().timestamp() {
                return Err(ProteinError::Authorization("MFA Code Expired".to_string()));
            }
        } else {
            return Err(ProteinError::Authorization(
                "MFA Code Potentially Expired".to_string(),
            ));
        }

        if let Some(ref mfa_code) = user.mfa_code {
            if *mfa_code == code {
                let result = User::reset_mfa(user_id, connection).await?;

                let receipent = result.clone();
                let mail = mailer.inner().clone();

                rocket::tokio::task::spawn(async move {
                    match email::send_email(
                        &mail,
                        &receipent,
                        "[Security] New Login",
                        format!(
                            "Hello {}! \n\nThere Has Been A New Login To Your Account Using MFA!",
                            receipent.username
                        ),
                    )
                    .await
                    {
                        Ok(_) => tracing::info!("[Email] ✅ Sent New Login Email"),
                        Err(e) => {
                            tracing::info!("[Email] ❌ Failed Sending New Login Email: {}", e)
                        }
                    }
                });

                return Ok(Json(json!(
                    {
                        "message": "MFA Code Verified",
                        "user": result
                    }
                )));
            } else {
                return Err(ProteinError::Authorization("Invalid MFA Code".to_string()));
            }
        } else {
            return Err(ProteinError::Authorization("No MFA Code".to_string()));
        }
    } else {
        return Err(ProteinError::Authorization("MFA Not Enabled".to_string()));
    }
}

#[get("/mfa/verify?<token>")]
pub async fn verify_mfa(
    _r: RateLimit<'_>,
    token: Uuid,
    pool: &State<DatabasePool>,
) -> Result<Json<Value>, ProteinError> {
    let connection = &mut db::get_connection(pool).await?;

    // Find User By Token
    let user = User::find_by_mfa_verification_token(token, connection).await?;

    if !user.mfa_verified && user.mfa_enabled {
        let verified_user = User::verify_mfa(user.user_id, connection).await?;

        Ok(Json(json!({
            "message": "MFA Verified Successfully",
            "user": verified_user,
        })))
    } else {
        Err(ProteinError::Email("MFA Already Verified".to_string()))
    }
}

#[get("/mfa/disable?<user_id>", format = "application/json")]
pub async fn disable_mfa(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DatabasePool>,
    mailer: &State<Mailer>,
    user_id: Uuid,
) -> Result<Json<User>, ProteinError> {
    let connection = &mut db::get_connection(pool).await?;

    let user = User::disable_mfa(user_id, connection).await?;

    let receipent = user.clone();
    let mail = mailer.inner().clone();

    rocket::tokio::task::spawn(async move {
        match email::send_email(&mail, &receipent, "[Security] MFA Disabled", format!("Hello {}! \n\nThis Email Serves As Notification For Disabling MFA For Your Account!", receipent.username)).await {
            Ok(_) => tracing::info!("[Email] ✅ Sent MFA Notification Email"),
            Err(e) => tracing::info!("[Email] ❌ Failed Sending MFA Notification Email: {}", e),
        }
    });

    Ok(Json(user))
}
