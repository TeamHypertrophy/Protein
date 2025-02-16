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
    responders::ProteinError,
    schema::{trainers, trainers::dsl::trainer_id},
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
#[diesel(primary_key(trainer_id))]
#[diesel(table_name = trainers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Trainer {
    trainer_id: i32,
    user_id: Uuid,
    clients: Vec<Option<Uuid>>,
    updated_at: NaiveDateTime,
}

impl Trainer {
    pub async fn find(
        trainer: i32,
        connection: &mut DatabaseConnection,
    ) -> Result<Trainer, ProteinError> {
        trainers::table
            .filter(trainer_id.eq(trainer))
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
        trainer: i32,
        data: UpdateTrainer,
        connection: &mut DatabaseConnection,
    ) -> Result<Trainer, ProteinError> {
        diesel::update(trainers::table)
            .filter(trainer_id.eq(trainer))
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
        trainer: i32,
        connection: &mut DatabaseConnection,
    ) -> Result<usize, ProteinError> {
        diesel::delete(trainers::table)
            .filter(trainer_id.eq(trainer))
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
}

#[derive(AsChangeset)]
#[diesel(table_name = trainers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
pub struct NewTrainer {
    pub user_id: Uuid,
    pub clients: Vec<Option<Uuid>>,
}
