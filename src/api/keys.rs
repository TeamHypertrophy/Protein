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
    serde::{
        json::{Json, Value, json},
        uuid::Uuid,
    },
};

use crate::{
    auth::{api::API, rate_limit::RateLimit},
    cache::redis::{Cache, Redis},
    db,
    db::DB,
    errors::Error,
    models::{
        keys::{
            APIKey, APIKeyLog, NewAPIKeyLog, RevokeKey, Status, UpdateAPIKey, UpdateAPIKeyLog,
            UpdateRole,
        },
        user::{Role, User},
    },
};

#[get("/get/<api_key>", format = "application/json")]
pub async fn get_api_key(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
    redis: &State<Redis>,
    api_key: Uuid,
) -> Result<Json<APIKey>, Error> {
    let connection = &mut db::get(pool).await?;

    let api_key = APIKey::find(api_key, connection).await?;

    // Store The API Key In Redis
    Cache::set(
        redis,
        "api_key",
        api_key.api_key.to_string(),
        Cache::serialize(&api_key)?,
    )
    .await?;

    Ok(Json(api_key))
}

#[get("/create?<user_id>", format = "application/json")]
pub async fn create_api_key(
    _r: RateLimit<'_>,
    pool: &State<DB>,
    redis: &State<Redis>,
    user_id: Uuid,
) -> Result<Json<APIKey>, Error> {
    let connection = &mut db::get(pool).await?;

    let user = User::find(user_id, connection).await?;

    let all = APIKey::user_all(user.user_id, connection).await?;

    for key in all {
        if key.status == Status::Active {
            return Err(Error::Authorization(
                "User already has an active API key".to_string(),
            ));
        }
    }

    let new = APIKey::generate(&user, connection).await?;

    // Store The API Key In Redis
    Cache::set(
        redis,
        "api_key",
        new.api_key.to_string(),
        Cache::serialize(&new)?,
    )
    .await?;

    Ok(Json(new))
}

#[get("/all", format = "application/json")]
pub async fn get_all_api_keys(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
) -> Result<Json<Vec<APIKey>>, Error> {
    let connection = &mut db::get(pool).await?;

    let api_keys = APIKey::all(connection).await?;

    Ok(Json(api_keys))
}

#[get("/user/all?<user_id>", format = "application/json")]
pub async fn get_all_user_api_keys(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
    user_id: Uuid,
) -> Result<Json<Vec<APIKey>>, Error> {
    let connection = &mut db::get(pool).await?;

    let user_keys = APIKey::user_all(user_id, connection).await?;

    Ok(Json(user_keys))
}

#[post("/update/<api_key>", format = "application/json", data = "<key>")]
pub async fn update_api_key(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
    api_key: Uuid,
    key: Json<UpdateAPIKey>,
) -> Result<Json<APIKey>, Error> {
    let connection = &mut db::get(pool).await?;

    let result = APIKey::update(api_key, key.into_inner(), connection).await?;

    Ok(Json(result))
}

#[post("/delete/<api_key>", format = "application/json")]
pub async fn delete_api_key(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
    api_key: Uuid,
) -> Result<status::Accepted<Value>, Error> {
    let connection = &mut db::get(pool).await?;

    APIKey::delete(api_key, connection).await?;

    Ok(status::Accepted(json!({
        "status": 200,
        "message": "API Key Revoked",
        "api_key": api_key
    })))
}

#[post("/revoke/<api_key>", format = "application/json", data = "<reason>")]
pub async fn revoke_api_key(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
    api_key: Uuid,
    reason: Json<RevokeKey>,
) -> Result<Json<APIKey>, Error> {
    let connection = &mut db::get(pool).await?;

    let result = APIKey::revoke(api_key, reason.into_inner(), connection).await?;

    Ok(Json(result))
}

#[post("/change-role/<api_key>", format = "application/json", data = "<role>")]
pub async fn change_api_key_role(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
    api_key: Uuid,
    role: Json<UpdateRole>,
) -> Result<Json<APIKey>, Error> {
    let connection = &mut db::get(pool).await?;

    let result = APIKey::change_role(api_key, role.into_inner(), connection).await?;

    Ok(Json(result))
}

#[get("/auth/is-admin?<api_key>", format = "application/json")]
pub async fn is_admin_key(
    _r: RateLimit<'_>,
    pool: &State<DB>,
    api_key: Uuid,
) -> Result<Json<Value>, Error> {
    let connection = &mut db::get(pool).await?;

    let key = APIKey::find(api_key, connection).await?;

    if key.role == Role::Admin || key.role == Role::Developer {
        Ok(Json(json!({
            "status": 200,
            "message": "Authorized",
            "api_key": api_key
        })))
    } else {
        Ok(Json(json!({
            "status": 403,
            "message": "Not Authorized",
            "api_key": api_key
        })))
    }
}

#[get("/logs/all", format = "application/json")]
pub async fn get_all_api_key_logs(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
) -> Result<Json<Vec<APIKeyLog>>, Error> {
    let connection = &mut db::get(pool).await?;

    let logs = APIKeyLog::all(connection).await?;

    Ok(Json(logs))
}

#[get("/logs/user/all?<user_id>", format = "application/json")]
pub async fn get_all_user_api_key_logs(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
    user_id: Uuid,
) -> Result<Json<Vec<APIKeyLog>>, Error> {
    let connection = &mut db::get(pool).await?;

    let logs = APIKeyLog::user_all(user_id, connection).await?;

    Ok(Json(logs))
}

#[post("/logs/create?<user_id>", format = "application/json", data = "<log>")]
pub async fn create_api_key_log(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
    user_id: Uuid,
    log: Json<NewAPIKeyLog>,
) -> Result<Json<APIKeyLog>, Error> {
    let connection = &mut db::get(pool).await?;

    let result = APIKeyLog::create(log.into_inner(), connection).await?;

    Ok(Json(result))
}

#[post("/logs/update/<log_id>", format = "application/json", data = "<log>")]
pub async fn update_api_key_log(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
    log_id: i32,
    log: Json<UpdateAPIKeyLog>,
) -> Result<Json<APIKeyLog>, Error> {
    let connection = &mut db::get(pool).await?;

    let result = APIKeyLog::update(log_id, log.into_inner(), connection).await?;

    Ok(Json(result))
}

#[post("/logs/delete/<log_id>", format = "application/json")]
pub async fn delete_api_key_log(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
    log_id: i32,
) -> Result<status::Accepted<Value>, Error> {
    let connection = &mut db::get(pool).await?;

    APIKeyLog::delete(log_id, connection).await?;

    Ok(status::Accepted(json!({
        "status": 200,
        "message": "API Key Log Deleted",
        "log_id": log_id
    })))
}
