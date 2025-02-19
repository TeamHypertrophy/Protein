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

use crate::{
    auth::rate_limit::RateLimit,
    cache::redis::{Cache, RedisPool},
    constants::PONG,
    db,
    db::DatabasePool,
    responders::ProteinError,
};

#[get("/redis", format = "application/json")]
pub async fn ping_redis(
    _r: RateLimit<'_>,
    redis: &State<RedisPool>,
) -> Result<Value, ProteinError> {
    // Run 'ping "PONG"'
    let ping: String = Cache::ping(redis, Some(PONG.to_owned())).await?;

    // Compare and Return
    if ping == *PONG {
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
pub async fn ping_postgres(
    _r: RateLimit<'_>,
    pool: &State<DatabasePool>,
) -> Result<Value, ProteinError> {
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
