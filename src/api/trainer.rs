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
    State, get, post,
    response::status,
    serde::json::{Json, Value, json},
};

use crate::{
    auth::{api::API, rate_limit::RateLimit},
    db,
    db::DB,
    errors::Error,
    models::{
        announcement::{NewTrainerAnnouncement, TrainerAnnouncement, UpdateTrainerAnnouncement},
        trainer::{NewTrainer, Trainer, UpdateTrainer},
    },
};

#[get("/get/<trainer_id>", format = "application/json")]
pub async fn get_trainer(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
    trainer_id: i32,
) -> Result<Json<Trainer>, Error> {
    let connection = &mut db::get_connection(pool).await?;

    let trainer = Trainer::find(trainer_id, connection).await?;

    Ok(Json(trainer))
}

#[get("/all", format = "application/json")]
pub async fn get_all_trainers(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
) -> Result<Json<Vec<Trainer>>, Error> {
    let connection = &mut db::get_connection(pool).await?;

    let trainers = Trainer::all(connection).await?;

    Ok(Json(trainers))
}

#[post("/update/<trainer_id>", format = "application/json", data = "<data>")]
pub async fn update_trainer(
    _r: RateLimit<'_>,
    pool: &State<DB>,
    trainer_id: i32,
    data: Json<UpdateTrainer>,
) -> Result<Json<Trainer>, Error> {
    let connection = &mut db::get_connection(pool).await?;

    let trainer = Trainer::update(trainer_id, data.into_inner(), connection).await?;

    Ok(Json(trainer))
}

#[post("/create", format = "application/json", data = "<data>")]
pub async fn create_trainer(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
    data: Json<NewTrainer>,
) -> Result<Json<Trainer>, Error> {
    let connection = &mut db::get_connection(pool).await?;

    let trainer = Trainer::create(data.into_inner(), connection).await?;

    Ok(Json(trainer))
}

#[post("/delete/<trainer_id>", format = "application/json")]
pub async fn delete_trainer(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
    trainer_id: i32,
) -> Result<status::Accepted<Value>, Error> {
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
    pool: &State<DB>,
    trainer_id: Uuid,
    announcement_id: i32,
) -> Result<Json<TrainerAnnouncement>, Error> {
    let connection = &mut db::get_connection(pool).await?;

    let announcement = TrainerAnnouncement::find(trainer_id, announcement_id, connection).await?;

    Ok(Json(announcement))
}

#[get("/announcement/all", format = "application/json")]
pub async fn get_all_announcements(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
) -> Result<Json<Vec<TrainerAnnouncement>>, Error> {
    let connection = &mut db::get_connection(pool).await?;

    let announcements = TrainerAnnouncement::all(connection).await?;

    Ok(Json(announcements))
}

#[get("/announcement/all?<trainer_id>", format = "application/json")]
pub async fn get_all_trainer_announcements(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
    trainer_id: Uuid,
) -> Result<Json<Vec<TrainerAnnouncement>>, Error> {
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
    pool: &State<DB>,
    trainer_id: Uuid,
    announcement_id: i32,
    data: Json<UpdateTrainerAnnouncement>,
) -> Result<Json<TrainerAnnouncement>, Error> {
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
    pool: &State<DB>,
    data: Json<NewTrainerAnnouncement>,
) -> Result<Json<TrainerAnnouncement>, Error> {
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
    pool: &State<DB>,
    trainer_id: Uuid,
    announcement_id: i32,
) -> Result<status::Accepted<Value>, Error> {
    let connection = &mut db::get_connection(pool).await?;

    TrainerAnnouncement::delete(trainer_id, announcement_id, connection).await?;

    Ok(status::Accepted(json!({
        "message": "Announcement Deleted Successfully",
    })))
}
