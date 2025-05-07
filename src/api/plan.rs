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
    db,
    db::DB,
    errors::Error,
    models::{
        plan::{
            NewWorkoutPlan, NewWorkoutPlanLog, UpdateWorkoutPlan, UpdateWorkoutPlanLog,
            WorkoutPlan, WorkoutPlanLog,
        },
        user::User,
    },
};

#[get("/get/<plan_id>?<user_id>", format = "application/json")]
pub async fn get_workout_plan(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
    user_id: Uuid,
    plan_id: Uuid,
) -> Result<Json<WorkoutPlan>, Error> {
    let connection = &mut db::get(pool).await?;

    let user = User::find(user_id, connection).await?;

    let plan = WorkoutPlan::find(&user, plan_id, connection).await?;

    Ok(Json(plan))
}

#[get("/all", format = "application/json")]
pub async fn get_all_workout_plans(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
) -> Result<Json<Vec<WorkoutPlan>>, Error> {
    let connection = &mut db::get(pool).await?;

    let plans = WorkoutPlan::all(connection).await?;

    Ok(Json(plans))
}

#[get("/user/all?<user_id>", format = "application/json")]
pub async fn get_all_user_workout_plans(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
    user_id: Uuid,
) -> Result<Json<Vec<WorkoutPlan>>, Error> {
    let connection = &mut db::get(pool).await?;

    let plans = WorkoutPlan::user_all(user_id, connection).await?;

    Ok(Json(plans))
}

#[post(
    "/update/<plan_id>?<user_id>",
    format = "application/json",
    data = "<data>"
)]
pub async fn update_workout_plan(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
    user_id: Uuid,
    plan_id: Uuid,
    data: Json<UpdateWorkoutPlan>,
) -> Result<Json<WorkoutPlan>, Error> {
    let connection = &mut db::get(pool).await?;

    let plan = WorkoutPlan::update(user_id, plan_id, data.into_inner(), connection).await?;

    Ok(Json(plan))
}

#[post("/create?<user_id>", format = "application/json", data = "<data>")]
pub async fn create_workout_plan(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
    user_id: Uuid,
    data: Json<NewWorkoutPlan>,
) -> Result<Json<WorkoutPlan>, Error> {
    let connection = &mut db::get(pool).await?;

    let plan = WorkoutPlan::create(data.into_inner(), connection).await?;

    Ok(Json(plan))
}

#[get("/delete/<plan_id>?<user_id>", format = "application/json")]
pub async fn delete_workout_plan(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
    user_id: Uuid,
    plan_id: Uuid,
) -> Result<status::Accepted<Value>, Error> {
    let connection = &mut db::get(pool).await?;

    WorkoutPlan::delete(user_id, plan_id, connection).await?;

    Ok(status::Accepted(json!({
        "status": 200,
        "message": "Workout Plan Deleted Successfully",
    })))
}

#[get("/log/<plan_id>?<user_id>", format = "application/json")]
pub async fn get_workout_plan_log(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
    user_id: Uuid,
    plan_id: Uuid,
) -> Result<Json<WorkoutPlanLog>, Error> {
    let connection = &mut db::get(pool).await?;

    let user = User::find(user_id, connection).await?;

    let log = WorkoutPlanLog::find(&user, plan_id, connection).await?;

    Ok(Json(log))
}

#[get("/log/all", format = "application/json")]
pub async fn get_all_workout_plan_logs(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
) -> Result<Json<Vec<WorkoutPlanLog>>, Error> {
    let connection = &mut db::get(pool).await?;

    let logs = WorkoutPlanLog::all(connection).await?;

    Ok(Json(logs))
}

#[get("/log/user/all?<user_id>", format = "application/json")]
pub async fn get_all_user_workout_plan_logs(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
    user_id: Uuid,
) -> Result<Json<Vec<WorkoutPlanLog>>, Error> {
    let connection = &mut db::get(pool).await?;

    let logs = WorkoutPlanLog::user_all(user_id, connection).await?;

    Ok(Json(logs))
}

#[post(
    "/log/update/<plan_id>?<user_id>",
    format = "application/json",
    data = "<data>"
)]
pub async fn update_workout_plan_log(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
    user_id: Uuid,
    plan_id: Uuid,
    data: Json<UpdateWorkoutPlanLog>,
) -> Result<Json<WorkoutPlanLog>, Error> {
    let connection = &mut db::get(pool).await?;

    let log = WorkoutPlanLog::update(user_id, plan_id, data.into_inner(), connection).await?;

    Ok(Json(log))
}

#[post("/log/create?<user_id>", format = "application/json", data = "<data>")]
pub async fn create_workout_plan_log(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
    user_id: Uuid,
    data: Json<NewWorkoutPlanLog>,
) -> Result<Json<WorkoutPlanLog>, Error> {
    let connection = &mut db::get(pool).await?;

    let log = WorkoutPlanLog::create(data.into_inner(), connection).await?;

    Ok(Json(log))
}

#[get("/log/delete/<plan_id>?<user_id>", format = "application/json")]
pub async fn delete_workout_plan_log(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
    user_id: Uuid,
    plan_id: Uuid,
) -> Result<status::Accepted<Value>, Error> {
    let connection = &mut db::get(pool).await?;

    WorkoutPlanLog::delete(user_id, plan_id, connection).await?;

    Ok(status::Accepted(json!({
        "status": 200,
        "message": "Workout Plan Log Deleted Successfully",
    })))
}
