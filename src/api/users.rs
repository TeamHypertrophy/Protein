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
    auth::dev::Developer,
    auth::key::API,
    cache::redis::{Cache, RedisPool},
    db,
    db::DatabasePool,
    models::keys::APIKey,
    models::user::{LoginUser, NewUser, ProteinUser, Role, UpdateUser, User},
    responders::ProteinError,
    utils::password,
};

#[get("/<user_id>", format = "application/json")]
pub async fn get(
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
    _auth: Developer,
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
    pool: &State<DatabasePool>,
    redis: &State<RedisPool>,
    user: Json<NewUser>,
) -> Result<Json<ProteinUser>, ProteinError> {
    let password_hash = password::generate_password(user.password.clone())?;

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
    let result = NewUser::create(connection, new_user).await?;

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

        return Ok(Json(ProteinUser {
            user: result,
            api_key: api_key.api_key,
        }));
    } else {
        Err(ProteinError::Authorization("Invalid Password".to_string()))
    }
}

#[get("/delete/<user_id>", format = "application/json")]
pub async fn delete(
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

#[post("/update/<user_id>", format = "application/json", data = "<user>")]
pub async fn update(
    _auth: API,
    user_id: Uuid,
    pool: &State<DatabasePool>,
    redis: &State<RedisPool>,
    user: Json<UpdateUser>,
) -> Result<Json<User>, ProteinError> {
    // Create Database Connection
    let connection = &mut db::get_connection(pool).await?;

    // Update User
    let updated_user = User::update(
        user_id,
        user.username.clone(),
        user.password.clone(),
        connection,
    )
    .await?;

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

#[get("/me", format = "application/json")]
pub async fn me(
    _auth: API,
    ip: &ClientRealAddr,
    pool: &State<DatabasePool>,
) -> Result<Json<User>, ProteinError> {
    // Get IP Address
    let ip_address: String = ip.get_ipv4_string().unwrap();

    // Create Database Connection
    let connection = &mut db::get_connection(pool).await?;

    // Grab User
    let user = User::find_by_ip(ip_address, connection).await?;

    Ok(Json(user))
}
