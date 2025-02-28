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
    models::protein::{NewProteinLog, ProteinLog, UpdateProteinLog},
};

#[get("/get/<log_id>?<user_id>", format = "application/json")]
pub async fn get_protein_log(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
    user_id: Uuid,
    log_id: i32,
) -> Result<Json<ProteinLog>, Error> {
    let connection = &mut db::get_connection(pool).await?;

    let log = ProteinLog::find(user_id, log_id, connection).await?;

    Ok(Json(log))
}

#[get("/all", format = "application/json")]
pub async fn get_all_protein_logs(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
) -> Result<Json<Vec<ProteinLog>>, Error> {
    let connection = &mut db::get_connection(pool).await?;

    let logs = ProteinLog::all(connection).await?;

    Ok(Json(logs))
}

#[get("/user/all?<user_id>", format = "application/json")]
pub async fn get_all_user_protein_logs(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
    user_id: Uuid,
) -> Result<Json<Vec<ProteinLog>>, Error> {
    let connection = &mut db::get_connection(pool).await?;

    let logs = ProteinLog::user_all(user_id, connection).await?;

    Ok(Json(logs))
}

#[post(
    "/update/<log_id>?<user_id>",
    format = "application/json",
    data = "<log>"
)]
pub async fn update_protein_log(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
    user_id: Uuid,
    log_id: i32,
    log: Json<UpdateProteinLog>,
) -> Result<Json<ProteinLog>, Error> {
    let connection = &mut db::get_connection(pool).await?;

    let result = ProteinLog::update(user_id, log_id, log.into_inner(), connection).await?;

    Ok(Json(result))
}

#[post("/create?<user_id>", format = "application/json", data = "<log>")]
pub async fn create_protein_log(
    _r: RateLimit<'_>,
    _auth: API,
    user_id: Uuid,
    pool: &State<DB>,
    log: Json<NewProteinLog>,
) -> Result<Json<ProteinLog>, Error> {
    let connection = &mut db::get_connection(pool).await?;

    let result = ProteinLog::create(log.into_inner(), connection).await?;

    Ok(Json(result))
}

#[post("/delete/<log_id>?<user_id>", format = "application/json")]
pub async fn delete_protein_log(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
    user_id: Uuid,
    log_id: i32,
) -> Result<status::Accepted<Value>, Error> {
    let connection = &mut db::get_connection(pool).await?;

    ProteinLog::delete(user_id, log_id, connection).await?;

    Ok(status::Accepted(json!({
        "status": 200,
        "message": "Log Deleted Successfully",
    })))
}
