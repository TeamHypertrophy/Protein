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
use rocket::{
    get, post,
    response::status,
    serde::{
        json::{json, Json, Value},
        uuid::Uuid,
    },
    State,
};

// Protein
use crate::{
    auth::key::API,
    auth::rate_limit::RateLimit,
    cache::redis::{Cache, RedisPool},
    db,
    db::DatabasePool,
    models::keys::APIKey,
    models::profile::{ForgotPassword, Profile},
    models::user::{LoginUser, NewUser, Password, ProteinUser, Role, UpdateUser, User},
    responders::ProteinError,
    utils::email,
    utils::email::Mailer,
    utils::password,
};

#[get("/?<user_id>", format = "application/json")]
pub async fn get(
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
        Cache::set(redis, "user", user_id.to_string(), Cache::serialize(&user)).await?;

        Ok(Json(user))
    } else {
        // Deserialize User
        let user: User = Cache::deserialize(cache);

        Ok(Json(user))
    }
}

#[get("/all", format = "application/json")]
pub async fn all(
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
    user: Json<NewUser>,
) -> Result<Json<ProteinUser>, ProteinError> {
    // Generate New User Password
    let password_hash = password::generate_hashed_password(user.password.clone())?;

    // Create New User Struct
    let new_user = NewUser {
        username: user.username.clone(),
        password: password_hash,
        role: user.role.clone(),
        ip_address: user.ip_address.clone(),
    };

    // Create Database Connection
    let connection = &mut db::get_connection(pool).await?;

    // Create User and Grab Result
    let result = User::create(connection, new_user).await?;

    // Generate API Key
    let api_key =
        APIKey::generate(&result, matches!(result.role, Role::Developer), connection).await?;

    // Set New User in Cache
    Cache::set(
        redis,
        "user",
        result.id.to_string(),
        Cache::serialize(&result),
    )
    .await?;

    // Set API Key In Cache
    Cache::set(
        redis,
        "api_key",
        api_key.api_key.to_string(),
        Cache::serialize(&api_key),
    )
    .await?;

    Ok(Json(ProteinUser {
        user: result,
        api_key: api_key.api_key,
    }))
}

#[post("/login", format = "application/json", data = "<user>")]
pub async fn login(
    _r: RateLimit<'_>,
    pool: &State<DatabasePool>,
    redis: &State<RedisPool>,
    user: Json<LoginUser>,
) -> Result<Json<ProteinUser>, ProteinError> {
    // Create Database Connection
    let connection = &mut db::get_connection(pool).await?;

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
            result.id.to_string(),
            Cache::serialize(&result),
        )
        .await?;

        // Set API Key in Cache
        Cache::set(
            redis,
            "api_key",
            api_key.api_key.to_string(),
            Cache::serialize(&api_key),
        )
        .await?;

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
pub async fn delete(
    _r: RateLimit<'_>,
    _auth: API,
    user_id: Uuid,
    pool: &State<DatabasePool>,
) -> Result<status::Accepted<Value>, ProteinError> {
    // Create Database Connection
    let connection = &mut db::get_connection(pool).await?;

    // Delete User
    User::delete(user_id, connection).await?;

    // Return Okay Message with Deleted ID
    Ok(status::Accepted(json!({
        "message": "User Deleted Successfully",
        "user_id": user_id,
    })))
}

#[post("/update?<user_id>", format = "application/json", data = "<user>")]
pub async fn update(
    _r: RateLimit<'_>,
    _auth: API,
    user_id: Uuid,
    pool: &State<DatabasePool>,
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
        Cache::serialize(&updated_user),
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
pub async fn update_password(
    _r: RateLimit<'_>,
    _auth: API,
    user_id: Uuid,
    pool: &State<DatabasePool>,
    redis: &State<RedisPool>,
    data: Json<Password>,
) -> Result<Json<User>, ProteinError> {
    // Create Database Connection
    let connection = &mut db::get_connection(pool).await?;

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
        Cache::serialize(&updated_user),
    )
    .await?;

    // Return Updated User
    Ok(Json(updated_user))
}

#[post("/email/forgot-password", format = "application/json", data = "<data>")]
pub async fn forgot_password_email(
    _r: RateLimit<'_>,
    mailer: &State<Mailer>,
    pool: &State<DatabasePool>,
    ip: &ClientRealAddr,
    data: Json<ForgotPassword>,
) -> Result<status::Accepted<Value>, ProteinError> {
    // Grab Database Connection
    let connection = &mut db::get_connection(pool).await?;

    // Get Current IP Address
    let ip_address: String = ip.get_ipv4_string().unwrap();

    // Get User Profile
    let profile = Profile::find_by_email(data.email.clone(), connection).await?;

    // Hash Generated Password
    let hashed_password = password::generate_hashed_password(data.password.clone())?;

    // Update User Password With New Hashed Data
    User::update_password(profile.user_id, &hashed_password, connection).await?;

    // Send Email To User
    email::send_email(mailer, &profile, "[Security] Your Password Has Been Reset", format!("Hello {}! \n\nThis Email Serves As Confirmation That Your Password Has Been Successfully Reset.\n\nNew Password: {}\n\n\nIf You Did NOT Request This, Please Ignore This Email\nRequest IP Address: {}", profile.first_name, data.password, ip_address)).await?;

    // API Response
    Ok(status::Accepted(json!({
        "message": "Forgot Password Email Sent!",
        "user_id": profile.user_id,
        "profile_id": profile.id
    })))
}

#[get("/me", format = "application/json")]
pub async fn me(
    _r: RateLimit<'_>,
    ip: &ClientRealAddr,
    pool: &State<DatabasePool>,
) -> Result<Json<Profile>, ProteinError> {
    // Get IP Address
    let ip_address: String = ip.get_ipv4_string().unwrap();

    // Create Database Connection
    let connection = &mut db::get_connection(pool).await?;

    // Grab User
    let user = User::find_by_ip(ip_address, connection).await?;

    // Grab Profile
    let profile = Profile::find(&user, connection).await?;

    Ok(Json(profile))
}
