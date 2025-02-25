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
use diesel_async::RunQueryDsl;
use rocket::serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

use crate::{
    db::DatabaseConnection,
    errors::ProteinError,
    schema::{
        trainer_announcements,
        trainer_announcements::dsl::{announcement_id, trainer_id},
    },
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
#[diesel(primary_key(announcement_id))]
#[diesel(table_name = trainer_announcements)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TrainerAnnouncement {
    pub announcement_id: i32,
    pub trainer_id: Uuid,
    pub title: String,
    pub visibility: bool,
    pub content: String,
    pub pinned: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl TrainerAnnouncement {
    pub async fn find(
        trainer: Uuid,
        announcement: i32,
        connection: &mut DatabaseConnection,
    ) -> Result<TrainerAnnouncement, ProteinError> {
        trainer_announcements::table
            .filter(trainer_id.eq(trainer))
            .filter(announcement_id.eq(announcement))
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
        trainer: Uuid,
        connection: &mut DatabaseConnection,
    ) -> Result<Vec<TrainerAnnouncement>, ProteinError> {
        trainer_announcements::table
            .filter(trainer_id.eq(trainer))
            .select(TrainerAnnouncement::as_select())
            .load(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                ProteinError::Database(error.to_string())
            })
    }

    pub async fn update(
        trainer: Uuid,
        announcement: i32,
        data: UpdateTrainerAnnouncement,
        connection: &mut DatabaseConnection,
    ) -> Result<TrainerAnnouncement, ProteinError> {
        diesel::update(trainer_announcements::table)
            .filter(trainer_id.eq(trainer))
            .filter(announcement_id.eq(announcement))
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
        announcement: i32,
        connection: &mut DatabaseConnection,
    ) -> Result<usize, ProteinError> {
        diesel::delete(trainer_announcements::table)
            .filter(trainer_id.eq(trainer))
            .filter(announcement_id.eq(announcement))
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
