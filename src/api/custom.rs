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
    cache::redis::{Cache, Group, Redis},
    db,
    db::DB,
    errors::Error,
    models::custom::{
        CustomExercise, NewCustomExercise, SearchCustomExercise, UpdateCustomExercise,
    },
};

#[get("/get/<exercise_id>?<user_id>", format = "application/json")]
pub async fn get_custom_exercise(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
    redis: &State<Redis>,
    user_id: Uuid,
    exercise_id: i64,
) -> Result<Json<CustomExercise>, Error> {
    let cache: Value = Cache::get(redis, Group::Custom, (user_id, exercise_id)).await?;

    if cache.is_null() {
        let connection = &mut db::get(pool).await?;

        let exercise: CustomExercise =
            CustomExercise::find(user_id, exercise_id, connection).await?;

        Cache::set(
            redis,
            Group::Custom,
            (user_id, exercise_id),
            Cache::serialize(&exercise)?,
        )
        .await?;

        Ok(Json(exercise))
    } else {
        let exercise: CustomExercise = Cache::deserialize(cache)?;

        Ok(Json(exercise))
    }
}

#[post("/search", format = "application/json", data = "<data>")]
pub async fn search_custom_exercises(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
    data: Json<SearchCustomExercise>,
) -> Result<Json<Vec<CustomExercise>>, Error> {
    let connection = &mut db::get(pool).await?;

    let exercises: Vec<CustomExercise> =
        CustomExercise::search(connection, data.into_inner()).await?;

    Ok(Json(exercises))
}

#[get("/all", format = "application/json")]
pub async fn get_all_custom_exercises(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
) -> Result<Json<Vec<CustomExercise>>, Error> {
    let connection = &mut db::get(pool).await?;

    let exercises: Vec<CustomExercise> = CustomExercise::all(connection).await?;

    Ok(Json(exercises))
}

#[get("/user/all?<user_id>", format = "application/json")]
pub async fn get_all_user_custom_exercises(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
    user_id: Uuid,
) -> Result<Json<Vec<CustomExercise>>, Error> {
    let connection = &mut db::get(pool).await?;

    let exercises: Vec<CustomExercise> = CustomExercise::user_all(user_id, connection).await?;

    Ok(Json(exercises))
}

#[post(
    "/update/<exercise_id>?<user_id>",
    format = "application/json",
    data = "<exercise>"
)]
pub async fn update_custom_exercise(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
    redis: &State<Redis>,
    user_id: Uuid,
    exercise_id: i64,
    exercise: Json<UpdateCustomExercise>,
) -> Result<Json<CustomExercise>, Error> {
    let connection = &mut db::get(pool).await?;

    let result: CustomExercise =
        CustomExercise::update(user_id, exercise_id, exercise.into_inner(), connection).await?;

    Cache::set(
        redis,
        Group::Custom,
        (user_id, exercise_id),
        Cache::serialize(&result)?,
    )
    .await?;

    Ok(Json(result))
}

#[post("/create?<user_id>", format = "application/json", data = "<exercise>")]
pub async fn create_custom_exercise(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
    redis: &State<Redis>,
    user_id: Uuid,
    exercise: Json<NewCustomExercise>,
) -> Result<Json<CustomExercise>, Error> {
    let connection = &mut db::get(pool).await?;

    let result: CustomExercise = CustomExercise::create(connection, exercise.into_inner()).await?;

    Cache::set(
        redis,
        Group::Custom,
        (user_id, result.exercise_id),
        Cache::serialize(&result)?,
    )
    .await?;

    Ok(Json(result))
}

#[get("/delete/<exercise_id>?<user_id>", format = "application/json")]
pub async fn delete_custom_exercise(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
    redis: &State<Redis>,
    user_id: Uuid,
    exercise_id: i64,
) -> Result<status::Accepted<Value>, Error> {
    let connection = &mut db::get(pool).await?;

    CustomExercise::delete(user_id, exercise_id, connection).await?;

    Cache::delete(redis, Group::Custom, (user_id, exercise_id)).await?;

    Ok(status::Accepted(json!({
        "status": 200,
        "message": "exercise Deleted Successfully",
    })))
}
