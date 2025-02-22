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
    db,
    db::DatabasePool,
    errors::ProteinError,
    models::sleep::{NewSleepLog, SleepLog, UpdateSleepLog},
};

#[get("/get/<log_id>?<user_id>", format = "application/json")]
pub async fn get_sleep_log(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DatabasePool>,
    user_id: Uuid,
    log_id: i32,
) -> Result<Json<SleepLog>, ProteinError> {
    let connection = &mut db::get_connection(pool).await?;

    let log = SleepLog::find(user_id, log_id, connection).await?;

    Ok(Json(log))
}

#[get("/all", format = "application/json")]
pub async fn get_all_sleep_logs(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DatabasePool>,
) -> Result<Json<Vec<SleepLog>>, ProteinError> {
    let connection = &mut db::get_connection(pool).await?;

    let logs = SleepLog::all(connection).await?;

    Ok(Json(logs))
}

#[get("/user/all?<user_id>", format = "application/json")]
pub async fn get_all_user_sleep_logs(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DatabasePool>,
    user_id: Uuid,
) -> Result<Json<Vec<SleepLog>>, ProteinError> {
    let connection = &mut db::get_connection(pool).await?;

    let logs = SleepLog::user_all(user_id, connection).await?;

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
    pool: &State<DatabasePool>,
    user_id: Uuid,
    log_id: i32,
    log: Json<UpdateSleepLog>,
) -> Result<Json<SleepLog>, ProteinError> {
    let connection = &mut db::get_connection(pool).await?;

    let result = SleepLog::update(user_id, log_id, log.into_inner(), connection).await?;

    Ok(Json(result))
}

#[post("/create?<user_id>", format = "application/json", data = "<log>")]
pub async fn create_sleep_log(
    _r: RateLimit<'_>,
    user_id: Uuid,
    pool: &State<DatabasePool>,
    log: Json<NewSleepLog>,
) -> Result<Json<SleepLog>, ProteinError> {
    let connection = &mut db::get_connection(pool).await?;

    let result = SleepLog::create(log.into_inner(), connection).await?;

    Ok(Json(result))
}

#[post("/delete/<log_id>?<user_id>", format = "application/json")]
pub async fn delete_sleep_log(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DatabasePool>,
    user_id: Uuid,
    log_id: i32,
) -> Result<status::Accepted<Value>, ProteinError> {
    let connection = &mut db::get_connection(pool).await?;

    SleepLog::delete(user_id, log_id, connection).await?;

    Ok(status::Accepted(json!({
        "message": "Log Deleted Successfully",
    })))
}
