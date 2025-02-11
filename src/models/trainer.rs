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
use diesel::prelude::*;
use diesel_derive_enum::DbEnum;
use diesel_async::RunQueryDsl;
use rocket::serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

use crate::{
    db::DatabaseConnection,
    responders::ProteinError,
    schema::{trainer_announcements, trainers},
};

#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    Queryable,
    Selectable,
    Serialize,
    Deserialize,
    Identifiable,
    Insertable,
)]
#[diesel(table_name = trainers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Trainer {
    id: i32,
    trainer_id: Uuid,
    clients: Vec<Option<Uuid>>,
    updated_at: NaiveDateTime,
}

impl Trainer {
    pub async fn find(
        trainer_id: Uuid,
        connection: &mut DatabaseConnection,
    ) -> Result<Trainer, ProteinError> {
        trainers::table
            .filter(trainers::dsl::trainer_id.eq(trainer_id))
            .first(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                ProteinError::Database(error.to_string())
            })
    }

    pub async fn all(connection: &mut DatabaseConnection) -> Result<Vec<Trainer>, ProteinError> {
        trainers::table
            .select(Trainer::as_select())
            .load(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                ProteinError::Database(error.to_string())
            })
    }

    pub async fn update(
        trainer_id: Uuid,
        data: UpdateTrainer,
        connection: &mut DatabaseConnection,
    ) -> Result<Trainer, ProteinError> {
        diesel::update(trainers::table)
            .filter(trainers::dsl::trainer_id.eq(trainer_id))
            .set(&data)
            .get_result::<Trainer>(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                ProteinError::Database(error.to_string())
            })
    }

    pub async fn create(
        data: NewTrainer,
        connection: &mut DatabaseConnection,
    ) -> Result<Trainer, ProteinError> {
        diesel::insert_into(trainers::table)
            .values(&data)
            .get_result::<Trainer>(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                ProteinError::Database(error.to_string())
            })
    }

    pub async fn delete(
        trainer: Uuid,
        connection: &mut DatabaseConnection,
    ) -> Result<usize, ProteinError> {
        diesel::delete(trainers::table)
            .filter(trainers::dsl::trainer_id.eq(trainer))
            .execute(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                ProteinError::Database(error.to_string())
            })
    }
}

#[derive(AsChangeset)]
#[diesel(table_name = trainers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTrainer {
    pub clients: Option<Vec<Option<Uuid>>>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(AsChangeset)]
#[diesel(table_name = trainers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
pub struct NewTrainer {
    pub trainer_id: Uuid,
    pub clients: Vec<Option<Uuid>>,
}

#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    Queryable,
    Selectable,
    Serialize,
    Deserialize,
    Identifiable,
    Insertable,
)]
#[diesel(table_name = trainer_announcements)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TrainerAnnouncement {
    id: i32,
    trainer_id: Uuid,
    title: String,
    content: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

impl TrainerAnnouncement {
    pub async fn find(
        trainer_id: Uuid,
        announcement_id: i32,
        connection: &mut DatabaseConnection,
    ) -> Result<TrainerAnnouncement, ProteinError> {
        trainer_announcements::table
            .filter(trainer_announcements::dsl::trainer_id.eq(trainer_id))
            .filter(trainer_announcements::dsl::id.eq(announcement_id))
            .first(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                ProteinError::Database(error.to_string())
            })
    }

    pub async fn all(
        connection: &mut DatabaseConnection,
    ) -> Result<Vec<TrainerAnnouncement>, ProteinError> {
        trainer_announcements::table
            .select(TrainerAnnouncement::as_select())
            .load(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                ProteinError::Database(error.to_string())
            })
    }

    pub async fn trainer_all(
        trainer_id: Uuid,
        connection: &mut DatabaseConnection,
    ) -> Result<Vec<TrainerAnnouncement>, ProteinError> {
        trainer_announcements::table
            .filter(trainer_announcements::dsl::trainer_id.eq(trainer_id))
            .select(TrainerAnnouncement::as_select())
            .load(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                ProteinError::Database(error.to_string())
            })
    }

    pub async fn update(
        trainer_id: Uuid,
        announcement_id: i32,
        data: UpdateTrainerAnnouncement,
        connection: &mut DatabaseConnection,
    ) -> Result<TrainerAnnouncement, ProteinError> {
        diesel::update(trainer_announcements::table)
            .filter(trainer_announcements::dsl::trainer_id.eq(trainer_id))
            .filter(trainer_announcements::dsl::id.eq(announcement_id))
            .set(&data)
            .get_result::<TrainerAnnouncement>(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                ProteinError::Database(error.to_string())
            })
    }

    pub async fn delete(
        trainer: Uuid,
        announcement_id: i32,
        connection: &mut DatabaseConnection,
    ) -> Result<usize, ProteinError> {
        diesel::delete(trainer_announcements::table)
            .filter(trainer_announcements::dsl::trainer_id.eq(trainer))
            .filter(trainer_announcements::dsl::id.eq(announcement_id))
            .execute(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                ProteinError::Database(error.to_string())
            })
    }

    pub async fn create(
        data: NewTrainerAnnouncement,
        connection: &mut DatabaseConnection,
    ) -> Result<TrainerAnnouncement, ProteinError> {
        diesel::insert_into(trainer_announcements::table)
            .values(&data)
            .get_result::<TrainerAnnouncement>(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                ProteinError::Database(error.to_string())
            })
    }
}

#[derive(AsChangeset)]
#[diesel(table_name = trainer_announcements)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTrainerAnnouncement {
    pub title: Option<String>,
    pub content: Option<String>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(AsChangeset)]
#[diesel(table_name = trainer_announcements)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
pub struct NewTrainerAnnouncement {
    pub trainer_id: Uuid,
    pub title: String,
    pub content: String,
}
