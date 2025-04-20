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
use askama::Template;
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
        profile::Profile,
        trainer::{NewTrainer, RequestTrainer, Specialization, Trainer, UpdateTrainer},
        user::User,
    },
    utils::{
        admin::Config,
        email,
        email::Email,
        templates::{TrainerAccepted, TrainerApplication, TrainerDenied, TrainerRequest},
    },
};

#[get("/get/<trainer_id>", format = "application/json")]
pub async fn get_trainer(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
    trainer_id: i32,
) -> Result<Json<Trainer>, Error> {
    let connection = &mut db::get(pool).await?;

    let trainer = Trainer::find(trainer_id, connection).await?;

    Ok(Json(trainer))
}

#[get("/all", format = "application/json")]
pub async fn get_all_trainers(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
) -> Result<Json<Vec<Trainer>>, Error> {
    let connection = &mut db::get(pool).await?;

    let trainers = Trainer::all(connection).await?;

    Ok(Json(trainers))
}

#[post("/update?<trainer_id>", format = "application/json", data = "<data>")]
pub async fn update_trainer(
    _r: RateLimit<'_>,
    pool: &State<DB>,
    trainer_id: i32,
    data: Json<UpdateTrainer>,
) -> Result<Json<Trainer>, Error> {
    let connection = &mut db::get(pool).await?;

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
    let connection = &mut db::get(pool).await?;

    let trainer = Trainer::create(data.into_inner(), connection).await?;

    Ok(Json(trainer))
}

#[post("/delete?<trainer_id>", format = "application/json")]
pub async fn delete_trainer(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
    trainer_id: i32,
) -> Result<status::Accepted<Value>, Error> {
    let connection = &mut db::get(pool).await?;

    Trainer::delete(trainer_id, connection).await?;

    Ok(status::Accepted(json!({
        "status": 200,
        "message": "Trainer Deleted Successfully",
    })))
}

#[get(
    "/announcement/<announcement_id>?<trainer_id>",
    format = "application/json"
)]
pub async fn get_announcement(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
    trainer_id: Uuid,
    announcement_id: i32,
) -> Result<Json<TrainerAnnouncement>, Error> {
    let connection = &mut db::get(pool).await?;

    let announcement = TrainerAnnouncement::find(trainer_id, announcement_id, connection).await?;

    Ok(Json(announcement))
}

#[get("/announcement/all", format = "application/json")]
pub async fn get_all_announcements(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
) -> Result<Json<Vec<TrainerAnnouncement>>, Error> {
    let connection = &mut db::get(pool).await?;

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
    let connection = &mut db::get(pool).await?;

    let announcements = TrainerAnnouncement::trainer_all(trainer_id, connection).await?;

    Ok(Json(announcements))
}

#[post(
    "/announcement/update/<announcement_id>?<trainer_id>",
    format = "application/json",
    data = "<data>"
)]
pub async fn update_announcement(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
    trainer_id: Uuid,
    announcement_id: i32,
    data: Json<UpdateTrainerAnnouncement>,
) -> Result<Json<TrainerAnnouncement>, Error> {
    let connection = &mut db::get(pool).await?;

    let announcement =
        TrainerAnnouncement::update(trainer_id, announcement_id, data.into_inner(), connection)
            .await?;

    Ok(Json(announcement))
}

#[post(
    "/announcement/create?<trainer_id>",
    format = "application/json",
    data = "<data>"
)]
pub async fn create_announcement(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
    data: Json<NewTrainerAnnouncement>,
    trainer_id: i32,
) -> Result<Json<TrainerAnnouncement>, Error> {
    let connection = &mut db::get(pool).await?;

    let announcement = TrainerAnnouncement::create(data.into_inner(), connection).await?;

    Ok(Json(announcement))
}

#[post(
    "/announcement/delete/<announcement_id>?<trainer_id>",
    format = "application/json"
)]
pub async fn delete_announcement(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
    trainer_id: Uuid,
    announcement_id: i32,
) -> Result<status::Accepted<Value>, Error> {
    let connection = &mut db::get(pool).await?;

    TrainerAnnouncement::delete(trainer_id, announcement_id, connection).await?;

    Ok(status::Accepted(json!({
        "status": 200,
        "message": "Announcement Deleted Successfully",
    })))
}

#[post(
    "/request/apply?<user_id>",
    format = "application/json",
    data = "<data>"
)]
pub async fn request_trainer(
    _r: RateLimit<'_>,
    _auth: API,
    pool: &State<DB>,
    mailer: &State<Email>,
    config: &State<Config>,
    user_id: Uuid,
    data: Json<RequestTrainer>,
) -> Result<Json<Trainer>, Error> {
    let connection = &mut db::get(pool).await?;

    let experience = data.experience.clone();
    let specialization = data.specialization.clone();

    let trainer = Trainer::request(data.into_inner(), connection).await?;

    let user = User::find(user_id, connection).await?;

    let profile = Profile::find(&user, connection).await?;

    let mail = mailer.inner().clone();
    let smtp = config.inner().clone();

    rocket::tokio::task::spawn(async move {
        let full_name = format!("{} {}", profile.first_name, profile.last_name);
        let trainer_id = trainer.trainer_id;

        let now = chrono::Utc::now().to_string();

        let accept_url = format!(
            "{}/trainers/request/accept?trainer_id={}",
            &smtp.host_url, &trainer_id
        );
        let deny_url = format!(
            "{}/trainers/request/deny?trainer_id={}",
            &smtp.host_url, &trainer_id
        );
        let status_url = format!("hypertrophy://trainer/status");

        let admin_body = TrainerApplication {
            name: &full_name,
            specialization: &specialization,
            time: &now,
            experience: &experience,
            accept_url: &accept_url,
            deny_url: &deny_url,
        };

        let user_body = TrainerRequest {
            name: &full_name,
            specialization: &specialization,
            time: &now,
            experience: &experience,
            status_url: &status_url,
        };

        match email::send(
            &mail,
            &smtp,
            &user,
            "[Trainer] Trainer Application Sent",
            user_body.render().unwrap(),
            false,
        )
        .await
        {
            Ok(_) => tracing::info!("[Email] ✅ Sent Trainer Application Request Email"),
            Err(e) => tracing::info!(
                "[Email] ❌ Failed Sending Trainer Application Request Email: {}",
                e
            ),
        };

        match email::send(
            &mail,
            &smtp,
            &user,
            "[Trainer] Trainer Application Received",
            admin_body.render().unwrap(),
            true,
        )
        .await
        {
            Ok(_) => tracing::info!("[Email] ✅ Sent Trainer Application Email"),
            Err(e) => tracing::info!("[Email] ❌ Failed Sending Trainer Application Email: {}", e),
        };
    });

    Ok(Json(trainer))
}

#[get("/request/accept?<trainer_id>")]
pub async fn accept_trainer(
    _r: RateLimit<'_>,
    pool: &State<DB>,
    mailer: &State<Email>,
    config: &State<Config>,
    trainer_id: i32,
) -> Result<Json<Trainer>, Error> {
    let connection = &mut db::get(pool).await?;

    let trainer = Trainer::accept(trainer_id, connection).await?;

    let user = User::find(trainer.user_id, connection).await?;

    let profile = Profile::find(&user, connection).await?;
    let full_name = format!("{} {}", profile.first_name, profile.last_name);

    let mail = mailer.inner().clone();
    let smtp = config.inner().clone();
    let specialization = match trainer.specialization {
        Specialization::WeightLoss => "Weight Loss",
        Specialization::MuscleGain => "Muscle Gain",
        Specialization::Maintenance => "Maintenance",
        Specialization::Endurance => "Endurance",
        Specialization::Strength => "Strength",
    };

    rocket::tokio::task::spawn(async move {
        let body = TrainerAccepted {
            name: &full_name,
            specialization: &specialization,
        };

        match email::send(
            &mail,
            &smtp,
            &user,
            "[Trainer] Trainer Application Accepted",
            body.render().unwrap(),
            false,
        )
        .await
        {
            Ok(_) => tracing::info!("[Email] ✅ Sent Trainer Application Accepted Email"),
            Err(e) => tracing::info!(
                "[Email] ❌ Failed Sending Trainer Application Accepted Email: {}",
                e
            ),
        };
    });

    Ok(Json(trainer))
}

#[get("/request/deny?<trainer_id>")]
pub async fn deny_trainer(
    _r: RateLimit<'_>,
    pool: &State<DB>,
    mailer: &State<Email>,
    config: &State<Config>,
    trainer_id: i32,
) -> Result<Json<Trainer>, Error> {
    let connection = &mut db::get(pool).await?;

    let trainer = Trainer::deny(trainer_id, connection).await?;

    let user = User::find(trainer.user_id, connection).await?;

    let mail = mailer.inner().clone();
    let smtp = config.inner().clone();

    rocket::tokio::task::spawn(async move {
        let body = TrainerDenied {
            name: &user.username,
        };

        match email::send(
            &mail,
            &smtp,
            &user,
            "[Trainer] Trainer Application Denied",
            body.render().unwrap(),
            false,
        )
        .await
        {
            Ok(_) => tracing::info!("[Email] ✅ Sent Trainer Application Denied Email"),
            Err(e) => tracing::info!(
                "[Email] ❌ Failed Sending Trainer Application Denied Email: {}",
                e
            ),
        };
    });

    Ok(Json(trainer))
}
