/*
______          _       _
| ___ \        | |     (_)
| |_/ / __ ___ | |_ ___ _ _ __
|  __/ '__/ _ \| __/ _ \ | '_ \
| |  | | | (_) | ||  __/ | | | |
\_|  |_|  \___/ \__\___|_|_| |_|

        Made with ❤️
*/


use rocket::{
    get,
    serde::json::{json, Value},
    State,
};

use crate::cache::redis::{Cache, RedisPool};

use crate::responders::ProteinError;

use crate::{db, db::DatabasePool};

#[get("/redis", format = "application/json")]
pub async fn redis(redis: &State<RedisPool>) -> Result<Value, ProteinError> {
    // Create Message To Test Connection
    let pong: String = String::from("PONG");

    // Run 'ping "PONG"'
    let ping: String = Cache::ping(redis, Some(pong.clone())).await?;

    // Compare and Return
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
