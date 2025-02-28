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
        user::User,
        workout::{NewWorkout, UpdateWorkout, Workout},
    },
};

#[get("/get/<workout_id>?<user_id>", format = "application/json")]
pub async fn get_workout(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
    user_id: Uuid,
    workout_id: Uuid,
) -> Result<Json<Workout>, Error> {
    let connection = &mut db::get_connection(pool).await?;

    let user = User::find(user_id, connection).await?;

    let workout = Workout::find(&user, workout_id, connection).await?;

    Ok(Json(workout))
}

#[get("/all", format = "application/json")]
pub async fn get_all_workouts(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
) -> Result<Json<Vec<Workout>>, Error> {
    let connection = &mut db::get_connection(pool).await?;

    let workouts = Workout::all(connection).await?;

    Ok(Json(workouts))
}

#[get("/user/all?<user_id>", format = "application/json")]
pub async fn get_all_user_workouts(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
    user_id: Uuid,
) -> Result<Json<Vec<Workout>>, Error> {
    let connection = &mut db::get_connection(pool).await?;

    let workouts = Workout::user_all(user_id, connection).await?;

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
    user_id: Uuid,
    workout_id: Uuid,
    data: Json<UpdateWorkout>,
) -> Result<Json<Workout>, Error> {
    let connection = &mut db::get_connection(pool).await?;

    let workout = Workout::update(user_id, workout_id, data.into_inner(), connection).await?;

    Ok(Json(workout))
}

#[post("/create?<user_id>", format = "application/json", data = "<data>")]
pub async fn create_workout(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
    user_id: Uuid,
    data: Json<NewWorkout>,
) -> Result<Json<Workout>, Error> {
    let connection = &mut db::get_connection(pool).await?;

    let workout = Workout::create(data.into_inner(), connection).await?;

    Ok(Json(workout))
}

#[post("/delete/<workout_id>?<user_id>", format = "application/json")]
pub async fn delete_workout(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
    user_id: Uuid,
    workout_id: Uuid,
) -> Result<status::Accepted<Value>, Error> {
    let connection = &mut db::get_connection(pool).await?;

    Workout::delete(user_id, workout_id, connection).await?;

    Ok(status::Accepted(json!({
        "message": "Log Deleted Successfully",
    })))
}
