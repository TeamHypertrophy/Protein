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
use diesel::{dsl::sql, prelude::*};
use diesel_async::RunQueryDsl;
use diesel_derive_enum::DbEnum;
use rocket::serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

use crate::{
    db::Conn,
    errors::Error,
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
    pub trainer_id: i32,
    pub user_id: Uuid,
    pub clients: Vec<Option<Uuid>>,
    pub specialization: Specialization,
    pub verified: bool,
    pub verified_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(DbEnum, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[ExistingTypePath = "crate::schema::sql_types::Specialization"]
pub enum Specialization {
    WeightLoss,
    MuscleGain,
    Maintenance,
    Endurance,
    Strength,
}

impl Trainer {
    pub async fn find(trainer: i32, connection: &mut Conn) -> Result<Trainer, Error> {
        trainers::table
            .filter(trainer_id.eq(trainer))
            .first(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn all(connection: &mut Conn) -> Result<Vec<Trainer>, Error> {
        trainers::table
            .select(Trainer::as_select())
            .load(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn update(
        trainer: i32,
        data: UpdateTrainer,
        connection: &mut Conn,
    ) -> Result<Trainer, Error> {
        diesel::update(trainers::table)
            .filter(trainer_id.eq(trainer))
            .set(&data)
            .get_result::<Trainer>(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn create(data: NewTrainer, connection: &mut Conn) -> Result<Trainer, Error> {
        diesel::insert_into(trainers::table)
            .values(&data)
            .get_result::<Trainer>(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn request(data: RequestTrainer, connection: &mut Conn) -> Result<Trainer, Error> {
        let trainer = NewTrainer {
            user_id: data.user_id,
            clients: vec![],
            specialization: match data.specialization.as_str() {
                "WeightLoss" => Specialization::WeightLoss,
                "MuscleGain" => Specialization::MuscleGain,
                "Maintenance" => Specialization::Maintenance,
                "Endurance" => Specialization::Endurance,
                "Strength" => Specialization::Strength,
                _ => Specialization::MuscleGain,
            },
        };

        diesel::insert_into(trainers::table)
            .values(&trainer)
            .get_result::<Trainer>(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn accept(trainer: i32, connection: &mut Conn) -> Result<Trainer, Error> {
        diesel::update(trainers::table)
            .filter(trainer_id.eq(trainer))
            .set((
                trainers::verified.eq(true),
                trainers::verified_at.eq(chrono::Utc::now().naive_utc()),
            ))
            .get_result::<Trainer>(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn deny(trainer: i32, connection: &mut Conn) -> Result<Trainer, Error> {
        diesel::delete(trainers::table)
            .filter(trainer_id.eq(trainer))
            .get_result::<Trainer>(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn delete(trainer: i32, connection: &mut Conn) -> Result<usize, Error> {
        diesel::delete(trainers::table)
            .filter(trainer_id.eq(trainer))
            .execute(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn for_user(user: Uuid, connection: &mut Conn) -> Result<Vec<Trainer>, Error> {
        let target = vec![Some(user)];

        trainers::table
            .filter(trainers::clients.contains(target))
            .select(Trainer::as_select())
            .load(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn by_user(user: Uuid, connection: &mut Conn) -> Result<Trainer, Error> {
        trainers::table
            .filter(trainers::user_id.eq(user))
            .first(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn add_client(
        trainer: Uuid,
        user: Uuid,
        connection: &mut Conn,
    ) -> Result<Trainer, Error> {
        diesel::update(trainers::table)
            .filter(trainers::user_id.eq(trainer))
            .set(trainers::clients.eq(sql(&format!("array_append(clients, '{}')", user))))
            .get_result::<Trainer>(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn remove_client(
        trainer: Uuid,
        user: Uuid,
        connection: &mut Conn,
    ) -> Result<Trainer, Error> {
        diesel::update(trainers::table)
            .filter(trainers::user_id.eq(trainer))
            .set(trainers::clients.eq(sql(&format!("array_remove(clients, '{}')", user))))
            .get_result::<Trainer>(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn is_followed_by_user(
        trainer: Uuid,
        user: Uuid,
        connection: &mut Conn,
    ) -> Result<bool, Error> {
        trainers::table
            .filter(trainers::user_id.eq(trainer))
            .filter(trainers::clients.contains(vec![Some(user)]))
            .select(sql::<diesel::sql_types::Bool>("true"))
            .first(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                Error::Database(error.to_string())
            })
    }
}

#[derive(AsChangeset)]
#[diesel(table_name = trainers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTrainer {
    pub clients: Option<Vec<Option<Uuid>>>,
    pub specialization: Option<Specialization>,
}

#[derive(AsChangeset)]
#[diesel(table_name = trainers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
pub struct NewTrainer {
    pub user_id: Uuid,
    pub clients: Vec<Option<Uuid>>,
    pub specialization: Specialization,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestTrainer {
    pub user_id: Uuid,
    pub specialization: String,
    pub experience: String,
}
