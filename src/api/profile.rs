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
    form::Form,
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
    cache::redis::{Cache, Group, Redis},
    constants::MAX_AVATAR_SIZE,
    db::{self, DB},
    errors::Error,
    models::{
        profile::{NewProfile, Profile, UpdateProfile},
        user::User,
    },
    utils::{admin::Config, password},
};

#[get("/?<user_id>", format = "application/json")]
pub async fn get_profile(
    _r: RateLimit<'_>,
    _auth: API,
    user_id: Uuid,
    pool: &State<DB>,
    redis: &State<Redis>,
) -> Result<Json<Profile>, Error> {
    // Check Cache
    let cache: Value = Cache::get(redis, Group::Profiles, user_id).await?;

    if cache.is_null() {
        // Create Database Connection
        let connection = &mut db::get(pool).await?;

        // Grab User
        let user = User::find(user_id, connection).await?;

        // Grab Profile
        let profile = Profile::find(&user, connection).await?;

        // Cache Profile
        Cache::set(redis, Group::Profiles, user_id, Cache::serialize(&profile)?).await?;

        Ok(Json(profile))
    } else {
        let profile: Profile = Cache::deserialize(cache)?;

        Ok(Json(profile))
    }
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
    pool: &State<DB>,
    redis: &State<Redis>,
    profile: Json<NewProfile>,
    user_id: Uuid,
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

    // Cache Profile
    Cache::set(redis, Group::Profiles, user_id, Cache::serialize(&result)?).await?;

    Ok(Json(result))
}

#[post("/update?<user_id>", format = "application/json", data = "<profile>")]
pub async fn update_profile(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
    redis: &State<Redis>,
    profile: Json<UpdateProfile>,
    user_id: Uuid,
) -> Result<Json<Profile>, Error> {
    let connection = &mut db::get(pool).await?;

    match profile.clone().into_inner().validate() {
        Ok(_) => (),
        Err(error) => return Err(Error::Validation(error.to_string())),
    }

    let result = Profile::update(user_id, profile.into_inner(), connection).await?;

    Cache::set(redis, Group::Profiles, user_id, Cache::serialize(&result)?).await?;

    Ok(Json(result))
}

#[post("/avatar/upload?<user_id>", data = "<avatar>")]
pub async fn upload_avatar(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
    redis: &State<Redis>,
    config: &State<Config>,
    mut avatar: Form<TempFile<'_>>,
    user_id: Uuid,
) -> Result<Json<Profile>, Error> {
    if avatar.len() > MAX_AVATAR_SIZE {
        return Err(Error::Validation("Avatar Size Exceeded".to_string()));
    }

    let directory = format!("assets/avatars/{}", user_id);

    match async_fs::create_dir(&directory).await {
        Ok(_) => (),
        Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => (),
        Err(error) => return Err(Error::IO(error.to_string())),
    }

    let avatar_id = password::random();

    let path = format!("assets/avatars/{}/{}.png", user_id, avatar_id);

    match avatar.persist_to(&path).await {
        Ok(_) => (),
        Err(error) => return Err(Error::IO(error.to_string())),
    }

    let url = format!(
        "{}/assets/avatars/{}/{}.png",
        config.avatar_host_url, user_id, avatar_id
    );

    let connection = &mut db::get(pool).await?;

    let profile = Profile::upload_avatar(user_id, url, connection).await?;

    Cache::set(redis, Group::Profiles, user_id, Cache::serialize(&profile)?).await?;

    Ok(Json(profile))
}

#[get("/delete/<user_id>", format = "application/json")]
pub async fn delete_profile(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
    redis: &State<Redis>,
    user_id: Uuid,
) -> Result<status::Accepted<Value>, Error> {
    let connection = &mut db::get(pool).await?;

    Profile::delete(user_id, connection).await?;

    Cache::delete(redis, Group::Profiles, user_id).await?;

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

#[get("/streak/increment?<user_id>", format = "application/json")]
pub async fn increment_streak(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
    redis: &State<Redis>,
    user_id: Uuid,
) -> Result<Json<Profile>, Error> {
    let connection = &mut db::get(pool).await?;

    let profile = Profile::increment_streak(user_id, connection).await?;

    Cache::set(redis, Group::Profiles, user_id, Cache::serialize(&profile)?).await?;

    Ok(Json(profile))
}

#[get("/streak/reset?<user_id>", format = "application/json")]
pub async fn reset_streak(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
    redis: &State<Redis>,
    user_id: Uuid,
) -> Result<Json<Profile>, Error> {
    let connection = &mut db::get(pool).await?;

    let profile = Profile::reset_streak(user_id, connection).await?;

    Cache::set(redis, Group::Profiles, user_id, Cache::serialize(&profile)?).await?;

    Ok(Json(profile))
}
