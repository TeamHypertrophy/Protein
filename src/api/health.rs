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
    State, get,
    serde::json::{Value, json},
};

use crate::{
    auth::rate_limit::RateLimit,
    cache::redis::{Cache, Redis},
    constants::PONG,
    db,
    db::DB,
    errors::Error,
};

#[get("/redis", format = "application/json")]
pub async fn ping_redis(_r: RateLimit<'_>, redis: &State<Redis>) -> Result<Value, Error> {
    // Run 'redis-server ping "PONG"'
    let ping: String = Cache::ping(redis, Some(PONG.to_owned())).await?;

    // Compare Redis Response
    if ping == *PONG {
        Ok(json!({
            "status": 200,
            "is_healthy": true
        }))
    } else {
        Ok(json!({
            "status": 500,
            "is_healthy": false
        }))
    }
}

#[get("/postgres", format = "application/json")]
pub async fn ping_postgres(_r: RateLimit<'_>, pool: &State<DB>) -> Result<Value, Error> {
    // Attempts To Fetech A Singular Connection From Database Pool
    match db::get(pool).await {
        Ok(_) => Ok(json!({
            "status": 200,
            "is_healthy": true
        })),
        Err(_) => Ok(json!({
            "status": 500,
            "is_healthy": false
        })),
    }
}
