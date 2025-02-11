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
    models::logs::{
        ExerciseLog, NewExerciseLog, NewWorkoutLog, UpdateExerciseLog, UpdateWorkoutLog, WorkoutLog,
    },
    responders::ProteinError,
};

// Exercises

#[get("/exercise/get/<exercise_id>?<user_id>", format = "application/json")]
pub async fn exercise_get(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DatabasePool>,
    exercise_id: i32,
    user_id: Uuid,
) -> Result<Json<ExerciseLog>, ProteinError> {
    let connection = &mut db::get_connection(pool).await?;

    let log = ExerciseLog::find(user_id, exercise_id, connection).await?;

    Ok(Json(log))
}

#[get("/exercise/all", format = "application/json")]
pub async fn exercise_all(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DatabasePool>,
) -> Result<Json<Vec<ExerciseLog>>, ProteinError> {
    let connection = &mut db::get_connection(pool).await?;

    let logs = ExerciseLog::all(connection).await?;

    Ok(Json(logs))
}

#[get("/exercise/user/all?<user_id>", format = "application/json")]
pub async fn exercise_user_all(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DatabasePool>,
    user_id: Uuid,
) -> Result<Json<Vec<ExerciseLog>>, ProteinError> {
    let connection = &mut db::get_connection(pool).await?;

    let logs = ExerciseLog::user_all(user_id, connection).await?;

    Ok(Json(logs))
}

#[post(
    "/exercise/update/<exercise_id>?<user_id>",
    format = "application/json",
    data = "<data>"
)]
pub async fn exercise_update(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DatabasePool>,
    exercise_id: i32,
    user_id: Uuid,
    data: Json<UpdateExerciseLog>,
) -> Result<Json<ExerciseLog>, ProteinError> {
    let connection = &mut db::get_connection(pool).await?;

    let log = ExerciseLog::update(user_id, exercise_id, data.into_inner(), connection).await?;

    Ok(Json(log))
}

#[post(
    "/exercise/create?<user_id>",
    format = "application/json",
    data = "<data>"
)]
pub async fn exercise_create(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DatabasePool>,
    user_id: Uuid,
    data: Json<NewExerciseLog>,
) -> Result<Json<ExerciseLog>, ProteinError> {
    let connection = &mut db::get_connection(pool).await?;

    let log = ExerciseLog::create(data.into_inner(), connection).await?;

    Ok(Json(log))
}

#[post(
    "/exercise/delete/<exercise_id>?<user_id>",
    format = "application/json"
)]
pub async fn exercise_delete(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DatabasePool>,
    exercise_id: i32,
    user_id: Uuid,
) -> Result<status::Accepted<Value>, ProteinError> {
    let connection = &mut db::get_connection(pool).await?;

    ExerciseLog::delete(user_id, exercise_id, connection).await?;

    Ok(status::Accepted(json!({
        "message": "Log Deleted Successfully",
    })))
}

// Workouts

#[get("/workout/get/<workout_id>?<user_id>", format = "application/json")]
pub async fn workout_get(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DatabasePool>,
    workout_id: Uuid,
    user_id: Uuid,
) -> Result<Json<WorkoutLog>, ProteinError> {
    let connection = &mut db::get_connection(pool).await?;

    let log = WorkoutLog::find(user_id, workout_id, connection).await?;

    Ok(Json(log))
}

#[get("/workout/all", format = "application/json")]
pub async fn workout_all(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DatabasePool>,
) -> Result<Json<Vec<WorkoutLog>>, ProteinError> {
    let connection = &mut db::get_connection(pool).await?;

    let logs = WorkoutLog::all(connection).await?;

    Ok(Json(logs))
}

#[get("/workout/user/all?<user_id>", format = "application/json")]
pub async fn workout_user_all(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DatabasePool>,
    user_id: Uuid,
) -> Result<Json<Vec<WorkoutLog>>, ProteinError> {
    let connection = &mut db::get_connection(pool).await?;

    let logs = WorkoutLog::user_all(user_id, connection).await?;

    Ok(Json(logs))
}

#[post(
    "/workout/update/<workout_id>?<user_id>",
    format = "application/json",
    data = "<data>"
)]
pub async fn workout_update(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DatabasePool>,
    workout_id: Uuid,
    user_id: Uuid,
    data: Json<UpdateWorkoutLog>,
) -> Result<Json<WorkoutLog>, ProteinError> {
    let connection = &mut db::get_connection(pool).await?;

    let log = WorkoutLog::update(user_id, workout_id, data.into_inner(), connection).await?;

    Ok(Json(log))
}

#[post(
    "/workout/create?<user_id>",
    format = "application/json",
    data = "<data>"
)]
pub async fn workout_create(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DatabasePool>,
    user_id: Uuid,
    data: Json<NewWorkoutLog>,
) -> Result<Json<WorkoutLog>, ProteinError> {
    let connection = &mut db::get_connection(pool).await?;

    let log = WorkoutLog::create(data.into_inner(), connection).await?;

    Ok(Json(log))
}

#[post("/workout/delete/<workout_id>?<user_id>", format = "application/json")]
pub async fn workout_delete(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DatabasePool>,
    workout_id: Uuid,
    user_id: Uuid,
) -> Result<status::Accepted<Value>, ProteinError> {
    let connection = &mut db::get_connection(pool).await?;

    WorkoutLog::delete(user_id, workout_id, connection).await?;

    Ok(status::Accepted(json!({
        "message": "Log Deleted Successfully",
    })))
}
