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
    State, get, post,
    response::status,
    serde::json::{Json, Value, json},
};
use uuid::Uuid;

use crate::{
    auth::{api::API, rate_limit::RateLimit},
    cache::redis::{Cache, Redis},
    db,
    db::DB,
    errors::Error,
    models::sleep::{NewSleepLog, SleepLog, UpdateSleepLog},
};

#[get("/get/<log_id>?<user_id>", format = "application/json")]
pub async fn get_sleep_log(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
    redis: &State<Redis>,
    user_id: Uuid,
    log_id: i32,
) -> Result<Json<SleepLog>, Error> {
    let cache: Value = Cache::get(redis, "sleep_logs", (user_id, log_id)).await?;

    if cache.is_null() {
        let connection = &mut db::get(pool).await?;

        let log: SleepLog = SleepLog::find(user_id, log_id, connection).await?;

        Cache::set(
            redis,
            "sleep_logs",
            (user_id, log_id),
            Cache::serialize(&log)?,
        )
        .await?;

        Ok(Json(log))
    } else {
        let log: SleepLog = Cache::deserialize(cache)?;

        Ok(Json(log))
    }
}

#[get("/all", format = "application/json")]
pub async fn get_all_sleep_logs(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
) -> Result<Json<Vec<SleepLog>>, Error> {
    let connection = &mut db::get(pool).await?;

    let logs: Vec<SleepLog> = SleepLog::all(connection).await?;

    Ok(Json(logs))
}

#[get("/user/all?<user_id>", format = "application/json")]
pub async fn get_all_user_sleep_logs(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
    user_id: Uuid,
) -> Result<Json<Vec<SleepLog>>, Error> {
    let connection = &mut db::get(pool).await?;

    let logs: Vec<SleepLog> = SleepLog::user_all(user_id, connection).await?;

    Ok(Json(logs))
}

#[post(
    "/update/<log_id>?<user_id>",
    format = "application/json",
    data = "<log>"
)]
pub async fn update_sleep_log(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
    redis: &State<Redis>,
    user_id: Uuid,
    log_id: i32,
    log: Json<UpdateSleepLog>,
) -> Result<Json<SleepLog>, Error> {
    let connection = &mut db::get(pool).await?;

    let result: SleepLog = SleepLog::update(user_id, log_id, log.into_inner(), connection).await?;

    Cache::set(
        redis,
        "sleep_logs",
        (user_id, log_id),
        Cache::serialize(&result)?,
    )
    .await?;

    Ok(Json(result))
}

#[post("/create?<user_id>", format = "application/json", data = "<log>")]
pub async fn create_sleep_log(
    _r: RateLimit<'_>,
    user_id: Uuid,
    pool: &State<DB>,
    redis: &State<Redis>,
    log: Json<NewSleepLog>,
) -> Result<Json<SleepLog>, Error> {
    let connection = &mut db::get(pool).await?;

    let result: SleepLog = SleepLog::create(log.into_inner(), connection).await?;

    Cache::set(
        redis,
        "sleep_logs",
        (user_id, result.log_id),
        Cache::serialize(&result)?,
    )
    .await?;

    Ok(Json(result))
}

#[get("/delete/<log_id>?<user_id>", format = "application/json")]
pub async fn delete_sleep_log(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
    redis: &State<Redis>,
    user_id: Uuid,
    log_id: i32,
) -> Result<status::Accepted<Value>, Error> {
    let connection = &mut db::get(pool).await?;

    SleepLog::delete(user_id, log_id, connection).await?;

    Cache::delete(redis, "sleep_logs", (user_id, log_id)).await?;

    Ok(status::Accepted(json!({
        "status": 200,
        "message": "Log Deleted Successfully",
    })))
}
