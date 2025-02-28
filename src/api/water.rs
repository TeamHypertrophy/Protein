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
    db::DB,
    errors::Error,
    models::water::{NewWaterLog, UpdateWaterLog, WaterLog},
};

#[get("/get/<log_id>?<user_id>", format = "application/json")]
pub async fn get_water_log(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
    user_id: Uuid,
    log_id: i32,
) -> Result<Json<WaterLog>, Error> {
    let connection = &mut db::get_connection(pool).await?;

    let log = WaterLog::find(user_id, log_id, connection).await?;

    Ok(Json(log))
}

#[get("/all", format = "application/json")]
pub async fn get_all_water_logs(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
) -> Result<Json<Vec<WaterLog>>, Error> {
    let connection = &mut db::get_connection(pool).await?;

    let logs = WaterLog::all(connection).await?;

    Ok(Json(logs))
}

#[get("/user/all?<user_id>", format = "application/json")]
pub async fn get_all_user_water_logs(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
    user_id: Uuid,
) -> Result<Json<Vec<WaterLog>>, Error> {
    let connection = &mut db::get_connection(pool).await?;

    let logs = WaterLog::user_all(user_id, connection).await?;

    Ok(Json(logs))
}

#[post(
    "/update/<log_id>?<user_id>",
    format = "application/json",
    data = "<log>"
)]
pub async fn update_water_log(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
    user_id: Uuid,
    log_id: i32,
    log: Json<UpdateWaterLog>,
) -> Result<Json<WaterLog>, Error> {
    let connection = &mut db::get_connection(pool).await?;

    let result = WaterLog::update(user_id, log_id, log.into_inner(), connection).await?;

    Ok(Json(result))
}

#[post("/create?<user_id>", format = "application/json", data = "<log>")]
pub async fn create_water_log(
    _r: RateLimit<'_>,
    _auth: API,
    user_id: Uuid,
    pool: &State<DB>,
    log: Json<NewWaterLog>,
) -> Result<Json<WaterLog>, Error> {
    let connection = &mut db::get_connection(pool).await?;

    let result = WaterLog::create(log.into_inner(), connection).await?;

    Ok(Json(result))
}

#[post("/delete/<log_id>?<user_id>", format = "application/json")]
pub async fn delete_water_log(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
    user_id: Uuid,
    log_id: i32,
) -> Result<status::Accepted<Value>, Error> {
    let connection = &mut db::get_connection(pool).await?;

    WaterLog::delete(user_id, log_id, connection).await?;

    Ok(status::Accepted(json!({
        "message": "Log Deleted Successfully",
    })))
}
