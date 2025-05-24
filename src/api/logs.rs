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
    models::logs::{
        ExerciseLog, NewExerciseLog, NewWorkoutLog, UpdateExerciseLog, UpdateWorkoutLog, WorkoutLog,
    },
};

// Exercises

#[get("/exercise/get/<exercise_id>?<user_id>", format = "application/json")]
pub async fn get_exercise_log(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
    redis: &State<Redis>,
    exercise_id: i32,
    user_id: Uuid,
) -> Result<Json<ExerciseLog>, Error> {
    let cache: Value = Cache::get(redis, Group::ExerciseLogs, (user_id, exercise_id)).await?;

    if cache.is_null() {
        let connection = &mut db::get(pool).await?;

        let log: ExerciseLog = ExerciseLog::find(user_id, exercise_id, connection).await?;

        Cache::set(
            redis,
            Group::ExerciseLogs,
            (user_id, exercise_id),
            Cache::serialize(&log)?,
        )
        .await?;

        Ok(Json(log))
    } else {
        let log: ExerciseLog = Cache::deserialize(cache)?;

        Ok(Json(log))
    }
}

#[get("/exercise/all", format = "application/json")]
pub async fn get_all_exercise_logs(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
) -> Result<Json<Vec<ExerciseLog>>, Error> {
    let connection = &mut db::get(pool).await?;

    let logs: Vec<ExerciseLog> = ExerciseLog::all(connection).await?;

    Ok(Json(logs))
}

#[get("/exercise/user/all?<user_id>", format = "application/json")]
pub async fn get_all_user_exercise_logs(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
    user_id: Uuid,
) -> Result<Json<Vec<ExerciseLog>>, Error> {
    let connection = &mut db::get(pool).await?;

    let logs: Vec<ExerciseLog> = ExerciseLog::user_all(user_id, connection).await?;

    Ok(Json(logs))
}

#[post(
    "/exercise/update/<exercise_id>?<user_id>",
    format = "application/json",
    data = "<data>"
)]
pub async fn update_exercise_log(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
    redis: &State<Redis>,
    exercise_id: i32,
    user_id: Uuid,
    data: Json<UpdateExerciseLog>,
) -> Result<Json<ExerciseLog>, Error> {
    let connection = &mut db::get(pool).await?;

    let log: ExerciseLog =
        ExerciseLog::update(user_id, exercise_id, data.into_inner(), connection).await?;

    Cache::set(
        redis,
        Group::ExerciseLogs,
        (user_id, exercise_id),
        Cache::serialize(&log)?,
    )
    .await?;

    Ok(Json(log))
}

#[post(
    "/exercise/create?<user_id>",
    format = "application/json",
    data = "<data>"
)]
pub async fn create_exercise_log(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
    redis: &State<Redis>,
    user_id: Uuid,
    data: Json<NewExerciseLog>,
) -> Result<Json<ExerciseLog>, Error> {
    let connection = &mut db::get(pool).await?;

    let log: ExerciseLog = ExerciseLog::create(data.into_inner(), connection).await?;

    Cache::set(
        redis,
        Group::ExerciseLogs,
        (user_id, log.log_id),
        Cache::serialize(&log)?,
    )
    .await?;

    Ok(Json(log))
}

#[get(
    "/exercise/delete/<exercise_id>?<user_id>",
    format = "application/json"
)]
pub async fn delete_exercise_log(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
    redis: &State<Redis>,
    exercise_id: i32,
    user_id: Uuid,
) -> Result<status::Accepted<Value>, Error> {
    let connection = &mut db::get(pool).await?;

    ExerciseLog::delete(user_id, exercise_id, connection).await?;

    Cache::delete(redis, Group::ExerciseLogs, (user_id, exercise_id)).await?;

    Ok(status::Accepted(json!({
        "status": 200,
        "message": "Log Deleted Successfully",
    })))
}

// Workouts

#[get("/workout/get/<log_id>?<user_id>", format = "application/json")]
pub async fn get_workout_log(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
    redis: &State<Redis>,
    log_id: i32,
    user_id: Uuid,
) -> Result<Json<WorkoutLog>, Error> {
    let cache: Value = Cache::get(redis, Group::WorkoutLogs, (user_id, log_id)).await?;

    if cache.is_null() {
        let connection = &mut db::get(pool).await?;

        let log: WorkoutLog = WorkoutLog::find(user_id, log_id, connection).await?;

        Cache::set(
            redis,
            Group::WorkoutLogs,
            (user_id, log_id),
            Cache::serialize(&log)?,
        )
        .await?;

        Ok(Json(log))
    } else {
        let log: WorkoutLog = Cache::deserialize(cache)?;

        Ok(Json(log))
    }
}

#[get("/workout/all", format = "application/json")]
pub async fn get_all_workout_logs(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
) -> Result<Json<Vec<WorkoutLog>>, Error> {
    let connection = &mut db::get(pool).await?;

    let logs: Vec<WorkoutLog> = WorkoutLog::all(connection).await?;

    Ok(Json(logs))
}

#[get("/workout/user/all?<user_id>", format = "application/json")]
pub async fn get_all_user_workout_logs(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
    user_id: Uuid,
) -> Result<Json<Vec<WorkoutLog>>, Error> {
    let connection = &mut db::get(pool).await?;

    let logs: Vec<WorkoutLog> = WorkoutLog::user_all(user_id, connection).await?;

    Ok(Json(logs))
}

#[post(
    "/workout/update/<log_id>?<user_id>",
    format = "application/json",
    data = "<data>"
)]
pub async fn update_workout_log(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
    redis: &State<Redis>,
    log_id: i32,
    user_id: Uuid,
    data: Json<UpdateWorkoutLog>,
) -> Result<Json<WorkoutLog>, Error> {
    let connection = &mut db::get(pool).await?;

    let log: WorkoutLog =
        WorkoutLog::update(user_id, log_id, data.into_inner(), connection).await?;

    Cache::set(
        redis,
        Group::WorkoutLogs,
        (user_id, log_id),
        Cache::serialize(&log)?,
    )
    .await?;

    Ok(Json(log))
}

#[post(
    "/workout/create?<user_id>",
    format = "application/json",
    data = "<data>"
)]
pub async fn create_workout_log(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
    redis: &State<Redis>,
    user_id: Uuid,
    data: Json<NewWorkoutLog>,
) -> Result<Json<WorkoutLog>, Error> {
    let connection = &mut db::get(pool).await?;

    let log: WorkoutLog = WorkoutLog::create(data.into_inner(), connection).await?;

    Cache::set(
        redis,
        Group::WorkoutLogs,
        (user_id, log.log_id),
        Cache::serialize(&log)?,
    )
    .await?;

    Ok(Json(log))
}

#[get("/workout/delete/<log_id>?<user_id>", format = "application/json")]
pub async fn delete_workout_log(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
    redis: &State<Redis>,
    log_id: i32,
    user_id: Uuid,
) -> Result<status::Accepted<Value>, Error> {
    let connection = &mut db::get(pool).await?;

    WorkoutLog::delete(user_id, log_id, connection).await?;

    Cache::delete(redis, Group::WorkoutLogs, (user_id, log_id)).await?;

    Ok(status::Accepted(json!({
        "status": 200,
        "message": "Log Deleted Successfully",
    })))
}
