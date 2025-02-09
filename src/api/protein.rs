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
    get, post,
    response::status,
    serde::json::{json, Json, Value},
    State,
};
use uuid::Uuid;

use crate::{
    auth::{key::API, rate_limit::RateLimit},
    db,
    db::DatabasePool,
    models::protein::{NewProteinLog, ProteinLog, UpdateProteinLog},
    responders::ProteinError,
};

#[get("/get/<log_id>?<user_id>", format = "application/json")]
pub async fn get(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DatabasePool>,
    user_id: Uuid,
    log_id: i32,
) -> Result<Json<ProteinLog>, ProteinError> {
    let connection = &mut db::get_connection(pool).await?;

    let log = ProteinLog::find(user_id, log_id, connection).await?;

    Ok(Json(log))
}

#[get("/all", format = "application/json")]
pub async fn all(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DatabasePool>,
) -> Result<Json<Vec<ProteinLog>>, ProteinError> {
    let connection = &mut db::get_connection(pool).await?;

    let logs = ProteinLog::all(connection).await?;

    Ok(Json(logs))
}

#[get("/user/all?<user_id>", format = "application/json")]
pub async fn user_all(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DatabasePool>,
    user_id: Uuid,
) -> Result<Json<Vec<ProteinLog>>, ProteinError> {
    let connection = &mut db::get_connection(pool).await?;

    let logs = ProteinLog::user_all(user_id, connection).await?;

    Ok(Json(logs))
}

#[post(
    "/update/<log_id>?<user_id>",
    format = "application/json",
    data = "<log>"
)]
pub async fn update(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DatabasePool>,
    user_id: Uuid,
    log_id: i32,
    log: Json<UpdateProteinLog>,
) -> Result<Json<ProteinLog>, ProteinError> {
    let connection = &mut db::get_connection(pool).await?;

    let result = ProteinLog::update(user_id, log_id, log.into_inner(), connection).await?;

    Ok(Json(result))
}

#[post("/create", format = "application/json", data = "<log>")]
pub async fn create(
    _r: RateLimit<'_>,
    pool: &State<DatabasePool>,
    log: Json<NewProteinLog>,
) -> Result<Json<ProteinLog>, ProteinError> {
    let connection = &mut db::get_connection(pool).await?;

    let result = NewProteinLog::create(log.into_inner(), connection).await?;

    Ok(Json(result))
}

#[post("/delete/<log_id>?<user_id>", format = "application/json")]
pub async fn delete(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DatabasePool>,
    user_id: Uuid,
    log_id: i32,
) -> Result<status::Accepted<Value>, ProteinError> {
    let connection = &mut db::get_connection(pool).await?;

    ProteinLog::delete(user_id, log_id, connection).await?;

    Ok(status::Accepted(json!({
        "message": "Log Deleted Successfully",
    })))
}
