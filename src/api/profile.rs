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
use validator::Validate;

use crate::{
    auth::{api::API, rate_limit::RateLimit},
    db,
    db::DatabasePool,
    errors::ProteinError,
    models::{
        profile::{NewProfile, Profile, UpdateProfile},
        user::User,
    },
};

#[get("/?<user_id>", format = "application/json")]
pub async fn get_profile(
    _r: RateLimit<'_>,
    _auth: API,
    user_id: Uuid,
    pool: &State<DatabasePool>,
) -> Result<Json<Profile>, ProteinError> {
    // Create Database Connection
    let connection = &mut db::get_connection(pool).await?;

    // Grab User
    let user = User::find(user_id, connection).await?;

    // Grab Profile
    let profile = Profile::find(&user, connection).await?;

    Ok(Json(profile))
}

#[get("/all", format = "application/json")]
pub async fn get_all_profiles(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DatabasePool>,
) -> Result<Json<Vec<Profile>>, ProteinError> {
    // Creating Database Connection
    let connection = &mut db::get_connection(pool).await?;

    // Grab All Profiles
    let profiles = Profile::all(connection).await?;

    Ok(Json(profiles))
}

#[post("/create?<user_id>", format = "application/json", data = "<profile>")]
pub async fn create_profile(
    _r: RateLimit<'_>,
    _auth: API,
    user_id: Uuid,
    pool: &State<DatabasePool>,
    profile: Json<NewProfile>,
) -> Result<Json<Profile>, ProteinError> {
    // Create Database Connection
    let connection = &mut db::get_connection(pool).await?;

    // Validation
    match profile.clone().into_inner().validate() {
        Ok(_) => (),
        Err(error) => return Err(ProteinError::Validation(error.to_string())),
    }

    // Create New Profile
    let result = Profile::create(profile.into_inner(), connection).await?;

    Ok(Json(result))
}

#[post("/update?<user_id>", format = "application/json", data = "<profile>")]
pub async fn update_profile(
    _r: RateLimit<'_>,
    _auth: API,
    user_id: Uuid,
    pool: &State<DatabasePool>,
    profile: Json<UpdateProfile>,
) -> Result<Json<Profile>, ProteinError> {
    let connection = &mut db::get_connection(pool).await?;

    match profile.clone().into_inner().validate() {
        Ok(_) => (),
        Err(error) => return Err(ProteinError::Validation(error.to_string())),
    }

    let result = Profile::update(user_id, profile.into_inner(), connection).await?;

    Ok(Json(result))
}

#[get("/delete/<user_id>", format = "application/json")]
pub async fn delete_profile(
    _r: RateLimit<'_>,
    _auth: API,
    user_id: Uuid,
    pool: &State<DatabasePool>,
) -> Result<status::Accepted<Value>, ProteinError> {
    let connection = &mut db::get_connection(pool).await?;

    Profile::delete(user_id, connection).await?;

    Ok(status::Accepted(json!({
        "message": "Profile Deleted Successfully",
        "user_id": user_id,
    })))
}

#[get("/leaderboard", format = "application/json")]
pub async fn get_leaderboard(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DatabasePool>,
) -> Result<Json<Vec<Profile>>, ProteinError> {
    let connection = &mut db::get_connection(pool).await?;

    let profiles = Profile::leaderboard(connection).await?;

    Ok(Json(profiles))
}
