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
use rocket::serde::uuid::Uuid;
use rocket::{get, post, State};

// Protein
use crate::{
    cache::redis::{RedisPool, Cache}, models::user::{NewUser, User}, db::DatabasePool, responders::ProteinError
};

#[get("/<user_id>", format = "application/json")]
pub async fn get(user_id: Uuid, pool: &State<DatabasePool>, redis: &State<RedisPool>) -> Result<Json<User>, ProteinError> {
    // Check Cache
    let cache: Value = Cache::get(redis, user_id.to_string())
        .await
        .map_err(|error| {
            tracing::error!("[!] Redis Error: {:?}", error);
            ProteinError::Cache(error.to_string())
        })?;

    // If Cache is Null, Fetch From Database
    if cache.is_null() {
        // Crate Database Connection
        let connection = &mut pool.get()
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error {:?}", error);
                ProteinError::Database(error.to_string())
            })?;
            
        // Grab User
        let user = User::find(user_id, connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error {:?}", error);
                ProteinError::Database(error.to_string())
            })?;

        // Set User in Cache
        let _ = Cache::set(redis, user_id.to_string(), Cache::serialize(&user))
            .await
            .map_err(|error| {
                tracing::error!("[!] Redis Error: {:?}", error);
                ProteinError::Cache(error.to_string())
            });
        
        Ok(Json(user))
    } else {
        // Deserialize User
        let user: User = Cache::deserialize(cache);

        Ok(Json(user))
    }
}

#[get("/all", format = "application/json")]
pub async fn all(pool: &State<DatabasePool>) -> Result<Json<Vec<User>>, ProteinError> {
    // Creating Database Connection
    let connection = &mut pool.get()
        .await
        .map_err(|error| {
            tracing::error!("[!] PostgreSQL Error {:?}", error);
            ProteinError::Database(error.to_string())
        })?;

    // Fetch List of Users and Return
    let users = User::all(connection)
        .await
        .map_err(|error| {
            tracing::error!("[!] PostgreSQL Error {:?}", error);
            ProteinError::Database(error.to_string())
        })?;
    
    Ok(Json(users))
}

#[post("/create", format = "application/json", data = "<user>")]
pub async fn create(pool: &State<DatabasePool>, redis: &State<RedisPool>, user: Json<NewUser>) -> Result<Json<User>, ProteinError> {
    // Create New User Struct
    let new_user = NewUser {
        username: user.username.clone(),
        is_dev: user.is_dev.clone()
    };

    // Create Database Connection
    let connection = &mut pool.get()
        .await
        .map_err(|error| {
            tracing::error!("[!] PostgreSQL Error {:?}", error);
            ProteinError::Database(error.to_string())
        })?;

    // Create User and Grab Result
    let result = NewUser::create(connection, new_user)
        .await
        .map_err(|error| {
            tracing::error!("[!] PostgreSQL Error {:?}", error);
            ProteinError::Database(error.to_string())
        })?;

    Ok(Json(result))
}

#[post("/delete/<user_id>", format = "application/json")]
pub async fn delete(user_id: Uuid, pool: &State<DatabasePool>) -> Json<User> {
    todo!()
}

#[post("/update/<user_id>", format = "application/json", data = "<user>")]
pub async fn update(user_id: Uuid, pool: &State<DatabasePool>, redis: &State<RedisPool>, user: Json<User>) -> Json<User> {
    todo!()
}