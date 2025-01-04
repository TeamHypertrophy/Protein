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
use crate::cache::redis::RedisPool;
use fred::interfaces::ClientLike;

// Database
use crate::db::DatabasePool;

#[get("/redis", format = "application/json")]
pub async fn redis(redis: &State<RedisPool>) -> Value {
    // [-] Create Message To Test Connection
    let health_message: String = String::from("PONG");

    // [-] Run 'ping "PONG"'
    let healthy: String = redis
        .ping(Some(health_message.clone()))
        .await
        .expect("[!] Could Not Ping Redis");

    // [>] Compare and Return
    if healthy == health_message {
        json!({
            "is_healthy": true
        })
    } else {
        json!({
            "is_healthy": false
        })
    }
}

#[get("/postgres", format = "application/json")]
pub async fn postgres(pool: &State<DatabasePool>) -> Value {
    // [-] Attempt to Create Database Connection
    let connection = &mut pool.get().await.ok();

    // [>] Compare and Return
    match connection {
        Some(_connection) => json!({
            "is_healthy": true
        }),
        None => json!({
            "is_healthy": false
        }),
    }
}
