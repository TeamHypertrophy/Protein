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
    models::profile::Profile,
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

    // Validate Email
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
        "{}/users/email/verify/{}",
        host, result.email_verification_token
    );

    email::send_email(mailer, &result, "[Security] Account Verification", format!("Hello {}! \n\nWelcome To Hypertrophy!\n\nWe Hope You Enjoy The Multitude of Features Provided\n\nPlease Verify Your Email At This Link:\n\n\n{}", result.username, verification_link)).await?;

    Ok(Json(ProteinUser {
        user: result,
        api_key: api_key.api_key,
    }))
}

#[get("/email/verify/<token>")]
pub async fn verify_email(
    _r: RateLimit<'_>,
    pool: &State<DatabasePool>,
    token: Uuid,
) -> Result<Json<Value>, ProteinError> {
    // Create Database Connection
    let connection = &mut db::get_connection(pool).await?;

    // Find User By Token
    let user = User::find_by_email_verification_token(token, connection).await?;

    if !user.email_verified {
        let verified_user = User::verify_email(user.user_id, connection).await?;

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
) -> Result<Json<ProteinUser>, ProteinError> {
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
        email::send_email(mailer, &result, "[Security] New Login", format!("Hello {}! \n\nThis Email Serves As Confirmation That There Has Been A New Login Into Your Account.\n\n\nIf You Did NOT Request This, Please Ignore This Email\nRequest IP Address: {}\nDevice: {}", user.username, ip_address, device)).await?;

        Ok(Json(ProteinUser {
            user: result,
            api_key: api_key.api_key,
        }))
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
    email::send_email(mailer, &user, "[Security] Your Account Has Been Deleted", format!("Hello {}! \n\nThis Email Serves As Confirmation That Your Account Has Been Successfully Deleted.\n\n\nIf You Did NOT Request This, Please Contact Support Immediately", user.username)).await?;

    // Return Okay Message with Deleted ID
    Ok(status::Accepted(json!({
        "message": "User Deleted Successfully",
        "user_id": user_id,
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
    webhook::send_audit_log(
        discord,
        "User Has Been Updated!",
        updated_user.username.as_str(),
        updated_user.last_login_ip.as_str(),
        "[User]",
    )
    .await?;

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
    email::send_email(mailer, &updated_user, "[Security] Your Password Has Been Updated", format!("Hello {}! \n\nThis Email Serves As Confirmation That Your Password Has Been Successfully Updated.\n\n\nIf You Did NOT Request This, Please Contact Support Immediately\n\nIP Address: {}", updated_user.username, updated_user.ip_address)).await?;

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
    email::send_email(mailer, &user, "[Security] Your Password Has Been Reset", format!("Hello {}! \n\nThis Email Serves As Confirmation That Your Password Has Been Successfully Reset.\n\nNew Password: {}\n\n\nIf You Did NOT Request This, Please Ignore This Email\nRequest IP Address: {}\nDevice: {}", user.username, data.password, ip_address, device)).await?;

    Ok(status::Accepted(json!({
        "message": "Forgot Password Email Sent!",
        "user_id": user.user_id,
    })))
}

#[get("/me", format = "application/json")]
pub async fn me(
    _r: RateLimit<'_>,
    ip: &ClientRealAddr,
    pool: &State<DatabasePool>,
) -> Result<Json<Profile>, ProteinError> {
    // Get IP Address
    let ip_address: String = match ip.get_ipv4_string() {
        Some(addr) => addr,
        None => return Err(ProteinError::Internal("Invalid IP Address".to_string())),
    };

    // Create Database Connection
    let connection = &mut db::get_connection(pool).await?;

    // Grab User
    let user = User::find_by_ip(ip_address, connection).await?;

    // Grab Profile
    let profile = Profile::find(&user, connection).await?;

    Ok(Json(profile))
}
