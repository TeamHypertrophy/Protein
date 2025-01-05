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
use rocket::serde::json::{json, Value};
use rocket::{get, State};

// Redis
use crate::cache::redis::{RedisPool, Cache};

// Errors
use crate::responders::ProteinError;

// Database
use crate::db;
use crate::db::DatabasePool;

#[get("/redis", format = "application/json")]
pub async fn redis(redis: &State<RedisPool>) -> Result<Value, ProteinError> {
    // [-] Create Message To Test Connection
    let pong: String = String::from("PONG");

    // [-] Run 'ping "PONG"'
    let ping: String = Cache::ping(redis, Some(pong.clone())).await?;

    // [>] Compare and Return
    if ping == pong {
        Ok(json!({
            "is_healthy": true
        }))
    } else {
        Ok(json!({
            "is_healthy": false
        }))
    }
}

#[get("/postgres", format = "application/json")]
pub async fn postgres(pool: &State<DatabasePool>) -> Result<Value, ProteinError> {
    // Verify Database Connectivity
    match db::get_connection(pool).await {
        Ok(_) => Ok(json!({
            "is_healthy": true
        })),
        Err(_) => Ok(json!({
            "is_healthy": false
        })),
    }
}
