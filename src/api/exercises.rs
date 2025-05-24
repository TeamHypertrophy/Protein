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
use validator::Validate;

use crate::{
    auth::{api::API, rate_limit::RateLimit},
    cache::redis::{Cache, Group, Redis},
    db,
    db::DB,
    errors::Error,
    models::exercise::{Exercise, NewExercise, SearchExercise, UpdateExercise},
};

#[get("/get/<exercise_id>", format = "application/json")]
pub async fn get_exercise(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
    redis: &State<Redis>,
    exercise_id: i64,
) -> Result<Json<Exercise>, Error> {
    let cache: Value = Cache::get(redis, Group::Exercises, exercise_id).await?;

    if cache.is_null() {
        let connection = &mut db::get(pool).await?;

        let exercise: Exercise = Exercise::find(exercise_id, connection).await?;

        Cache::set(
            redis,
            Group::Exercises,
            exercise_id,
            Cache::serialize(&exercise)?,
        )
        .await?;

        Ok(Json(exercise))
    } else {
        let exercise: Exercise = Cache::deserialize(cache)?;

        Ok(Json(exercise))
    }
}

#[post("/search", format = "application/json", data = "<data>")]
pub async fn search_exercises(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
    data: Json<SearchExercise>,
) -> Result<Json<Vec<Exercise>>, Error> {
    let connection = &mut db::get(pool).await?;

    let exercises: Vec<Exercise> = Exercise::search(connection, data.into_inner()).await?;

    Ok(Json(exercises))
}

#[get("/all", format = "application/json")]
pub async fn get_all_exercises(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
) -> Result<Json<Vec<Exercise>>, Error> {
    let connection = &mut db::get(pool).await?;

    let exercises: Vec<Exercise> = Exercise::all(connection).await?;

    Ok(Json(exercises))
}

#[post("/update/<exercise_id>", format = "application/json", data = "<data>")]
pub async fn update_exercise(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
    redis: &State<Redis>,
    exercise_id: i64,
    data: Json<UpdateExercise>,
) -> Result<Json<Exercise>, Error> {
    let connection = &mut db::get(pool).await?;

    match data.clone().into_inner().validate() {
        Ok(_) => (),
        Err(error) => return Err(Error::Validation(error.to_string())),
    }

    let exercise: Exercise = Exercise::update(exercise_id, data.into_inner(), connection).await?;

    Cache::set(
        redis,
        Group::Exercises,
        exercise_id,
        Cache::serialize(&exercise)?,
    )
    .await?;

    Ok(Json(exercise))
}

#[post("/create", format = "application/json", data = "<data>")]
pub async fn create_exercise(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
    redis: &State<Redis>,
    data: Json<NewExercise>,
) -> Result<Json<Exercise>, Error> {
    let connection = &mut db::get(pool).await?;

    match data.clone().into_inner().validate() {
        Ok(_) => (),
        Err(error) => return Err(Error::Validation(error.to_string())),
    }

    let exercise: Exercise = Exercise::create(connection, data.into_inner()).await?;

    Cache::set(
        redis,
        Group::Exercises,
        exercise.exercise_id,
        Cache::serialize(&exercise)?,
    )
    .await?;

    Ok(Json(exercise))
}

#[get("/delete/<exercise_id>", format = "application/json")]
pub async fn delete_exercise(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
    redis: &State<Redis>,
    exercise_id: i64,
) -> Result<status::Accepted<Value>, Error> {
    let connection = &mut db::get(pool).await?;

    Exercise::delete(exercise_id, connection).await?;

    Cache::delete(redis, Group::Exercises, exercise_id).await?;

    Ok(status::Accepted(json!({
        "status": 200,
        "message": "Exercise Deleted Successfully",
        "exercise_id": exercise_id,
    })))
}
