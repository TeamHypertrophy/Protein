/*
______          _       _
| ___ \        | |     (_)
| |_/ / __ ___ | |_ ___ _ _ __
|  __/ '__/ _ \| __/ _ \ | '_ \
| |  | | | (_) | ||  __/ | | | |
\_|  |_|  \___/ \__\___|_|_| |_|

        Made with ❤️
*/

use uuid::Uuid;
use rocket::{
    get, post,
    response::status,
    serde::json::{json, Json, Value},
    State,
};

use crate::{
    auth::{key::API, rate_limit::RateLimit},
    db,
    db::DatabasePool,
    models::{
        announcement::{NewTrainerAnnouncement, TrainerAnnouncement, UpdateTrainerAnnouncement},
        trainer::{NewTrainer, Trainer, UpdateTrainer},
    },
    responders::ProteinError,
};

#[get("/get/<trainer_id>", format = "application/json")]
pub async fn get_trainer(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DatabasePool>,
    trainer_id: i32,
) -> Result<Json<Trainer>, ProteinError> {
    let connection = &mut db::get_connection(pool).await?;

    let trainer = Trainer::find(trainer_id, connection).await?;

    Ok(Json(trainer))
}

#[get("/all", format = "application/json")]
pub async fn get_all_trainers(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DatabasePool>,
) -> Result<Json<Vec<Trainer>>, ProteinError> {
    let connection = &mut db::get_connection(pool).await?;

    let trainers = Trainer::all(connection).await?;

    Ok(Json(trainers))
}

#[post("/update/<trainer_id>", format = "application/json", data = "<data>")]
pub async fn update_trainer(
    _r: RateLimit<'_>,
    pool: &State<DatabasePool>,
    trainer_id: i32,
    data: Json<UpdateTrainer>,
) -> Result<Json<Trainer>, ProteinError> {
    let connection = &mut db::get_connection(pool).await?;

    let trainer = Trainer::update(trainer_id, data.into_inner(), connection).await?;

    Ok(Json(trainer))
}

#[post("/create", format = "application/json", data = "<data>")]
pub async fn create_trainer(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DatabasePool>,
    data: Json<NewTrainer>,
) -> Result<Json<Trainer>, ProteinError> {
    let connection = &mut db::get_connection(pool).await?;

    let trainer = Trainer::create(data.into_inner(), connection).await?;

    Ok(Json(trainer))
}

#[post("/delete/<trainer_id>", format = "application/json")]
pub async fn delete_trainer(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DatabasePool>,
    trainer_id: i32,
) -> Result<status::Accepted<Value>, ProteinError> {
    let connection = &mut db::get_connection(pool).await?;

    Trainer::delete(trainer_id, connection).await?;

    Ok(status::Accepted(json!({
        "message": "Trainer Deleted Successfully",
    })))
}

#[get(
    "/announcement/<trainer_id>?<announcement_id>",
    format = "application/json"
)]
pub async fn get_announcement(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DatabasePool>,
    trainer_id: Uuid,
    announcement_id: i32,
) -> Result<Json<TrainerAnnouncement>, ProteinError> {
    let connection = &mut db::get_connection(pool).await?;

    let announcement = TrainerAnnouncement::find(trainer_id, announcement_id, connection).await?;

    Ok(Json(announcement))
}

#[get("/announcement/all", format = "application/json")]
pub async fn get_all_announcements(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DatabasePool>,
) -> Result<Json<Vec<TrainerAnnouncement>>, ProteinError> {
    let connection = &mut db::get_connection(pool).await?;

    let announcements = TrainerAnnouncement::all(connection).await?;

    Ok(Json(announcements))
}

#[get("/announcement/all?<trainer_id>", format = "application/json")]
pub async fn get_all_trainer_announcements(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DatabasePool>,
    trainer_id: Uuid,
) -> Result<Json<Vec<TrainerAnnouncement>>, ProteinError> {
    let connection = &mut db::get_connection(pool).await?;

    let announcements = TrainerAnnouncement::trainer_all(trainer_id, connection).await?;

    Ok(Json(announcements))
}

#[post(
    "/announcement/update/<trainer_id>?<announcement_id>",
    format = "application/json",
    data = "<data>"
)]
pub async fn update_announcement(
    _r: RateLimit<'_>,
    pool: &State<DatabasePool>,
    trainer_id: Uuid,
    announcement_id: i32,
    data: Json<UpdateTrainerAnnouncement>,
) -> Result<Json<TrainerAnnouncement>, ProteinError> {
    let connection = &mut db::get_connection(pool).await?;

    let announcement =
        TrainerAnnouncement::update(trainer_id, announcement_id, data.into_inner(), connection)
            .await?;

    Ok(Json(announcement))
}

#[post("/announcement/create", format = "application/json", data = "<data>")]
pub async fn create_announcement(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DatabasePool>,
    data: Json<NewTrainerAnnouncement>,
) -> Result<Json<TrainerAnnouncement>, ProteinError> {
    let connection = &mut db::get_connection(pool).await?;

    let announcement = TrainerAnnouncement::create(data.into_inner(), connection).await?;

    Ok(Json(announcement))
}

#[post(
    "/announcement/delete/<trainer_id>?<announcement_id>",
    format = "application/json"
)]
pub async fn delete_announcement(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DatabasePool>,
    trainer_id: Uuid,
    announcement_id: i32,
) -> Result<status::Accepted<Value>, ProteinError> {
    let connection = &mut db::get_connection(pool).await?;

    TrainerAnnouncement::delete(trainer_id, announcement_id, connection).await?;

    Ok(status::Accepted(json!({
        "message": "Announcement Deleted Successfully",
    })))
}
