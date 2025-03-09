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
    State,
    fs::TempFile,
    get, post,
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
    db::DB,
    errors::Error,
    models::{
        profile::{NewProfile, Profile, UpdateProfile},
        user::User,
    },
    utils::password,
};

#[get("/?<user_id>", format = "application/json")]
pub async fn get_profile(
    _r: RateLimit<'_>,
    _auth: API,
    user_id: Uuid,
    pool: &State<DB>,
) -> Result<Json<Profile>, Error> {
    // Create Database Connection
    let connection = &mut db::get(pool).await?;

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
    pool: &State<DB>,
) -> Result<Json<Vec<Profile>>, Error> {
    // Creating Database Connection
    let connection = &mut db::get(pool).await?;

    // Grab All Profiles
    let profiles = Profile::all(connection).await?;

    Ok(Json(profiles))
}

#[post("/create?<user_id>", format = "application/json", data = "<profile>")]
pub async fn create_profile(
    _r: RateLimit<'_>,
    _auth: API,
    user_id: Uuid,
    pool: &State<DB>,
    profile: Json<NewProfile>,
) -> Result<Json<Profile>, Error> {
    // Create Database Connection
    let connection = &mut db::get(pool).await?;

    // Validation
    match profile.clone().into_inner().validate() {
        Ok(_) => (),
        Err(error) => return Err(Error::Validation(error.to_string())),
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
    pool: &State<DB>,
    profile: Json<UpdateProfile>,
) -> Result<Json<Profile>, Error> {
    let connection = &mut db::get(pool).await?;

    match profile.clone().into_inner().validate() {
        Ok(_) => (),
        Err(error) => return Err(Error::Validation(error.to_string())),
    }

    let result = Profile::update(user_id, profile.into_inner(), connection).await?;

    Ok(Json(result))
}

#[post("/avatar/upload?<user_id>", data = "<avatar>")]
pub async fn upload_avatar(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
    user_id: Uuid,
    mut avatar: TempFile<'_>,
) -> Result<Json<Profile>, Error> {
    let directory = format!("assets/avatars/{}", user_id.to_string());

    match async_fs::create_dir(&directory).await {
        Ok(_) => (),
        Err(error) => return Err(Error::IO(error.to_string())),
    }

    let avatar_id = password::random();

    let path = format!("assets/avatars/{}/{}.png", user_id.to_string(), avatar_id);

    match avatar.persist_to(&path).await {
        Ok(_) => (),
        Err(error) => return Err(Error::IO(error.to_string())),
    }

    let host =
        std::env::var("AVATAR_HOST_URL").unwrap_or_else(|_| "http://localhost:8000/".to_string());

    let url = format!(
        "{}/assets/avatars/{}/{}.png",
        host,
        user_id.to_string(),
        avatar_id
    );

    let connection = &mut db::get(pool).await?;

    let profile = Profile::upload_avatar(user_id, url, connection).await?;

    Ok(Json(profile))
}

#[get("/delete/<user_id>", format = "application/json")]
pub async fn delete_profile(
    _r: RateLimit<'_>,
    _auth: API,
    user_id: Uuid,
    pool: &State<DB>,
) -> Result<status::Accepted<Value>, Error> {
    let connection = &mut db::get(pool).await?;

    Profile::delete(user_id, connection).await?;

    Ok(status::Accepted(json!({
        "status": 200,
        "message": "Profile Deleted Successfully",
        "user_id": user_id,
    })))
}

#[get("/leaderboard", format = "application/json")]
pub async fn get_leaderboard(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
) -> Result<Json<Vec<Profile>>, Error> {
    let connection = &mut db::get(pool).await?;

    let profiles = Profile::leaderboard(connection).await?;

    Ok(Json(profiles))
}
