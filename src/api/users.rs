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
use rocket::serde::json::{Json, Value};
use rocket::serde::json::json;
use rocket::response::status;
use rocket::serde::uuid::Uuid;
use rocket::{get, post, State};

// Protein
use crate::{
    cache::redis::{RedisPool, Cache},
    models::user::{NewUser, CreatedUser, UpdateUser, User, Me},
    models::keys::APIKey,
    db,
    db::DatabasePool,
    responders::ProteinError,
    auth::Auth,
};

#[get("/<user_id>", format = "application/json")]
pub async fn get(
    _auth: Auth,
    user_id: Uuid,
    pool: &State<DatabasePool>,
    redis: &State<RedisPool>,
) -> Result<Json<User>, ProteinError> {
    // Check Cache
    let cache: Value = Cache::get(redis, format!("user:{}", user_id)).await?;

    // If Cache is Null, Fetch From Database
    if cache.is_null() {
        // Crate Database Connection
        let connection = &mut db::get_connection(pool).await?;

        // Grab User
        let user = User::find(user_id, connection).await?;

        // Set User in Cache
        Cache::set(redis, format!("user:{}", user_id), Cache::serialize(&user)).await?;

        Ok(Json(user))
    } else {
        // Deserialize User
        let user: User = Cache::deserialize(cache);

        Ok(Json(user))
    }
}

#[get("/all", format = "application/json")]
pub async fn all(_auth: Auth, pool: &State<DatabasePool>) -> Result<Json<Vec<User>>, ProteinError> {
    // Creating Database Connection
    let connection = &mut db::get_connection(pool).await?;

    // Fetch List of Users and Return
    let users = User::all(connection).await?;

    Ok(Json(users))
}

#[post("/create", format = "application/json", data = "<user>")]
pub async fn create(
    pool: &State<DatabasePool>,
    redis: &State<RedisPool>,
    user: Json<NewUser>,
) -> Result<Json<CreatedUser>, ProteinError> {
    // Create New User Struct
    let new_user = NewUser {
        username: user.username.clone(),
        is_dev: user.is_dev.clone(),
    };

    // Create Database Connection
    let connection = &mut db::get_connection(pool).await?;

    // Create User and Grab Result
    let result = NewUser::create(connection, new_user).await?;

    // Generate API Key
    let api_key = APIKey::generate(&result, connection).await?;

    // Set New User in Cache
    Cache::set(
        redis,
        format!("user:{}", result.id),
        Cache::serialize(&result),
    )
    .await?;

    Ok(Json(CreatedUser {
        user: result,
        api_key: api_key.api_key,
    }))
}

#[get("/delete/<user_id>", format = "application/json")]
pub async fn delete(
    _auth: Auth,
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
    _auth: Auth,
    user_id: Uuid,
    pool: &State<DatabasePool>,
    redis: &State<RedisPool>,
    user: Json<UpdateUser>,
) -> Result<Json<User>, ProteinError> {
    // Create Database Connection
    let connection = &mut db::get_connection(pool).await?;

    // Update User
    let updated_user = User::update(user_id, user.username.clone(), connection).await?;

    // Update Cache With User
    Cache::set(
        redis,
        format!("user:{}", user_id),
        Cache::serialize(&updated_user),
    )
    .await?;

    // Return Updated User
    Ok(Json(updated_user))
}

#[get("/me/<user_id>", format = "application/json")]
pub async fn me(
    _auth: Auth,
    user_id: Uuid,
    pool: &State<DatabasePool>,
) -> Result<Json<Me>, ProteinError> {
    // Create Database Connection
    let connection = &mut db::get_connection(pool).await?;

    // Grab User
    let user = User::find(user_id, connection).await?;

    // Grab API Key
    let api_key = APIKey::get(&user, connection).await?;

    Ok(Json(Me {
        user: user,
        api_key: api_key.clone(),
    }))
}
