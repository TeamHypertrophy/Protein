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
    db,
    db::DatabasePool,
    models::keys::{APIKey, RevokeKey, UpdateAPIKey, UpdateRole},
    responders::ProteinError,
};

#[get("/get/<api_key>", format = "application/json")]
pub async fn get_api_key(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DatabasePool>,
    api_key: Uuid,
) -> Result<Json<APIKey>, ProteinError> {
    let connection = &mut db::get_connection(pool).await?;

    let api_key = APIKey::find(api_key, connection).await?;

    Ok(Json(api_key))
}

#[get("/all", format = "application/json")]
pub async fn get_all_api_keys(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DatabasePool>,
) -> Result<Json<Vec<APIKey>>, ProteinError> {
    let connection = &mut db::get_connection(pool).await?;

    let api_keys = APIKey::all(connection).await?;

    Ok(Json(api_keys))
}

#[get("/all?<user_id>", format = "application/json")]
pub async fn get_all_user_api_keys(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DatabasePool>,
    user_id: Uuid,
) -> Result<Json<Vec<APIKey>>, ProteinError> {
    let connection = &mut db::get_connection(pool).await?;

    let user_keys = APIKey::user_all(user_id, connection).await?;

    Ok(Json(user_keys))
}

#[post("/update/<api_key>", format = "application/json", data = "<key>")]
pub async fn update_api_key(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DatabasePool>,
    api_key: Uuid,
    key: Json<UpdateAPIKey>,
) -> Result<Json<APIKey>, ProteinError> {
    let connection = &mut db::get_connection(pool).await?;

    let result = APIKey::update(api_key, key.into_inner(), connection).await?;

    Ok(Json(result))
}

#[post("/delete/<api_key>", format = "application/json")]
pub async fn delete_api_key(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DatabasePool>,
    api_key: Uuid,
) -> Result<status::Accepted<Value>, ProteinError> {
    let connection = &mut db::get_connection(pool).await?;

    APIKey::delete(api_key, connection).await?;

    Ok(status::Accepted(json!({
        "message": "API Key Revoked",
        "api_key": api_key
    })))
}

#[post("/revoke/<api_key>", format = "application/json", data = "<reason>")]
pub async fn revoke_api_key(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DatabasePool>,
    api_key: Uuid,
    reason: Json<RevokeKey>,
) -> Result<Json<APIKey>, ProteinError> {
    let connection = &mut db::get_connection(pool).await?;

    let result = APIKey::revoke(api_key, reason.into_inner(), connection).await?;

    Ok(Json(result))
}

#[post("/change-role/<api_key>", format = "application/json", data = "<role>")]
pub async fn change_api_key_role(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DatabasePool>,
    api_key: Uuid,
    role: Json<UpdateRole>,
) -> Result<Json<APIKey>, ProteinError> {
    let connection = &mut db::get_connection(pool).await?;

    let result = APIKey::change_role(api_key, role.into_inner(), connection).await?;

    Ok(Json(result))
}
