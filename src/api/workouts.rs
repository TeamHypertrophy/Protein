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
    models::{
        user::User,
        workout::{ExerciseID, NewWorkout, UpdateWorkout, Workout},
    },
};

#[get("/get/<workout_id>?<user_id>", format = "application/json")]
pub async fn get_workout(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
    redis: &State<Redis>,
    user_id: Uuid,
    workout_id: Uuid,
) -> Result<Json<Workout>, Error> {
    let cache: Value = Cache::get(redis, Group::Workouts, (user_id, workout_id)).await?;

    if cache.is_null() {
        let connection = &mut db::get(pool).await?;

        let user: User = User::find(user_id, connection).await?;

        let workout: Workout = Workout::find(&user, workout_id, connection).await?;

        Cache::set(
            redis,
            Group::Workouts,
            (user_id, workout_id),
            Cache::serialize(&workout)?,
        )
        .await?;

        Ok(Json(workout))
    } else {
        let workout: Workout = Cache::deserialize(cache)?;

        Ok(Json(workout))
    }
}

#[get("/all", format = "application/json")]
pub async fn get_all_workouts(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
) -> Result<Json<Vec<Workout>>, Error> {
    let connection = &mut db::get(pool).await?;

    let workouts: Vec<Workout> = Workout::all(connection).await?;

    Ok(Json(workouts))
}

#[get("/user/all?<user_id>", format = "application/json")]
pub async fn get_all_user_workouts(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
    user_id: Uuid,
) -> Result<Json<Vec<Workout>>, Error> {
    let connection = &mut db::get(pool).await?;

    let workouts: Vec<Workout> = Workout::user_all(user_id, connection).await?;

    Ok(Json(workouts))
}

#[post(
    "/update/<workout_id>?<user_id>",
    format = "application/json",
    data = "<data>"
)]
pub async fn update_workout(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
    redis: &State<Redis>,
    user_id: Uuid,
    workout_id: Uuid,
    data: Json<UpdateWorkout>,
) -> Result<Json<Workout>, Error> {
    let connection = &mut db::get(pool).await?;

    let workout: Workout =
        Workout::update(user_id, workout_id, data.into_inner(), connection).await?;

    Cache::set(
        redis,
        Group::Workouts,
        (user_id, workout_id),
        Cache::serialize(&workout)?,
    )
    .await?;

    Ok(Json(workout))
}

#[post("/create?<user_id>", format = "application/json", data = "<data>")]
pub async fn create_workout(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
    redis: &State<Redis>,
    user_id: Uuid,
    data: Json<NewWorkout>,
) -> Result<Json<Workout>, Error> {
    let connection = &mut db::get(pool).await?;

    let workout: Workout = Workout::create(data.into_inner(), connection).await?;

    Cache::set(
        redis,
        Group::Workouts,
        (user_id, workout.workout_id),
        Cache::serialize(&workout)?,
    )
    .await?;

    Ok(Json(workout))
}

#[get("/delete/<workout_id>?<user_id>", format = "application/json")]
pub async fn delete_workout(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
    redis: &State<Redis>,
    user_id: Uuid,
    workout_id: Uuid,
) -> Result<status::Accepted<Value>, Error> {
    let connection = &mut db::get(pool).await?;

    Workout::delete(user_id, workout_id, connection).await?;

    Cache::delete(redis, Group::Workouts, (user_id, workout_id)).await?;

    Ok(status::Accepted(json!({
        "status": 200,
        "message": "Log Deleted Successfully",
    })))
}

#[post(
    "/add-exercise/<workout_id>?<user_id>",
    format = "application/json",
    data = "<exercise>"
)]
pub async fn add_exercise(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
    redis: &State<Redis>,
    user_id: Uuid,
    workout_id: Uuid,
    exercise: Json<ExerciseID>,
) -> Result<Json<Workout>, Error> {
    let connection = &mut db::get(pool).await?;

    let workout: Workout =
        Workout::add_exercise(user_id, workout_id, exercise.into_inner(), connection).await?;

    Cache::set(
        redis,
        Group::Workouts,
        (user_id, workout_id),
        Cache::serialize(&workout)?,
    )
    .await?;

    Ok(Json(workout))
}

#[post(
    "/remove-exercise/<workout_id>?<user_id>",
    format = "application/json",
    data = "<exercise>"
)]
pub async fn remove_exercise(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
    redis: &State<Redis>,
    user_id: Uuid,
    workout_id: Uuid,
    exercise: Json<ExerciseID>,
) -> Result<Json<Workout>, Error> {
    let connection = &mut db::get(pool).await?;

    let workout: Workout =
        Workout::remove_exercise(user_id, workout_id, exercise.into_inner(), connection).await?;

    Cache::set(
        redis,
        Group::Workouts,
        (user_id, workout_id),
        Cache::serialize(&workout)?,
    )
    .await?;

    Ok(Json(workout))
}
