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
    db,
    db::DatabasePool,
    errors::ProteinError,
    models::exercise::{Exercise, NewExercise, SearchExercise, UpdateExercise},
};

#[get("/get/<exercise_id>", format = "application/json")]
pub async fn get_exercise(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DatabasePool>,
    exercise_id: i64,
) -> Result<Json<Exercise>, ProteinError> {
    let connection = &mut db::get_connection(pool).await?;

    let exercise = Exercise::find(exercise_id, connection).await?;

    Ok(Json(exercise))
}

#[post("/search", format = "application/json", data = "<data>")]
pub async fn search_exercises(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DatabasePool>,
    data: Json<SearchExercise>,
) -> Result<Json<Vec<Exercise>>, ProteinError> {
    let connection = &mut db::get_connection(pool).await?;

    let exercises = Exercise::search(connection, data.into_inner()).await?;

    Ok(Json(exercises))
}

#[get("/all", format = "application/json")]
pub async fn get_all_exercises(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DatabasePool>,
) -> Result<Json<Vec<Exercise>>, ProteinError> {
    let connection = &mut db::get_connection(pool).await?;

    let exercises = Exercise::all(connection).await?;

    Ok(Json(exercises))
}

#[post("/update/<exercise_id>", format = "application/json", data = "<data>")]
pub async fn update_exercise(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DatabasePool>,
    exercise_id: i64,
    data: Json<UpdateExercise>,
) -> Result<Json<Exercise>, ProteinError> {
    let connection = &mut db::get_connection(pool).await?;

    match data.clone().into_inner().validate() {
        Ok(_) => (),
        Err(error) => return Err(ProteinError::Validation(error.to_string())),
    }

    let exercise = Exercise::update(exercise_id, data.into_inner(), connection).await?;

    Ok(Json(exercise))
}

#[post("/create", format = "application/json", data = "<data>")]
pub async fn create_exercise(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DatabasePool>,
    data: Json<NewExercise>,
) -> Result<Json<Exercise>, ProteinError> {
    let connection = &mut db::get_connection(pool).await?;

    match data.clone().into_inner().validate() {
        Ok(_) => (),
        Err(error) => return Err(ProteinError::Validation(error.to_string())),
    }

    let exercise = Exercise::create(connection, data.into_inner()).await?;

    Ok(Json(exercise))
}

#[post("/delete/<exercise_id>", format = "application/json")]
pub async fn delete_exercise(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DatabasePool>,
    exercise_id: i64,
) -> Result<status::Accepted<Value>, ProteinError> {
    let connection = &mut db::get_connection(pool).await?;

    Exercise::delete(exercise_id, connection).await?;

    Ok(status::Accepted(json!({
        "message": "Exercise Deleted Successfully",
        "exercise_id": exercise_id,
    })))
}
