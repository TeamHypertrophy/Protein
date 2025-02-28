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
    db::DBConnection,
    errors::Error,
    models::user::User,
    schema::{
        workouts,
        workouts::dsl::{user_id, workout_id},
    },
};

#[derive(DbEnum, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[ExistingTypePath = "crate::schema::sql_types::Difficulty"]
pub enum Difficulty {
    Beginner,
    Intermediate,
    Advanced,
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
    Associations,
)]
#[diesel(primary_key(workout_id))]
#[diesel(table_name = workouts)]
#[diesel(belongs_to(User))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Workout {
    pub workout_id: Uuid,
    pub name: String,
    pub description: String,
    pub duration: i32,
    pub difficulty: Difficulty,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub user_id: Uuid,
    pub exercises: Vec<Option<i64>>,
}

impl Workout {
    pub async fn find(
        user: &User,
        id: Uuid,
        connection: &mut DBConnection,
    ) -> Result<Workout, Error> {
        Workout::belonging_to(user)
            .select(Workout::as_select())
            .filter(workout_id.eq(id))
            .first(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn all(connection: &mut DBConnection) -> Result<Vec<Workout>, Error> {
        workouts::table
            .select(Workout::as_select())
            .load(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn user_all(
        user: Uuid,
        connection: &mut DBConnection,
    ) -> Result<Vec<Workout>, Error> {
        workouts::table
            .filter(user_id.eq(user))
            .select(Workout::as_select())
            .load(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn update(
        user: Uuid,
        id: Uuid,
        data: UpdateWorkout,
        connection: &mut DBConnection,
    ) -> Result<Workout, Error> {
        diesel::update(workouts::table)
            .filter(user_id.eq(user))
            .filter(workout_id.eq(id))
            .set(&data)
            .get_result::<Workout>(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn delete(
        user: Uuid,
        id: Uuid,
        connection: &mut DBConnection,
    ) -> Result<usize, Error> {
        diesel::delete(workouts::table)
            .filter(user_id.eq(user))
            .filter(workout_id.eq(id))
            .execute(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn create(data: NewWorkout, connection: &mut DBConnection) -> Result<Workout, Error> {
        diesel::insert_into(workouts::table)
            .values(&data)
            .get_result::<Workout>(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                Error::Database(error.to_string())
            })
    }
}

#[derive(AsChangeset)]
#[diesel(table_name = workouts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateWorkout {
    pub name: Option<String>,
    pub description: Option<String>,
    pub duration: Option<i32>,
    pub difficulty: Option<Difficulty>,
    pub exercises: Option<Vec<Option<i64>>>,
}

#[derive(AsChangeset)]
#[diesel(table_name = workouts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
pub struct NewWorkout {
    pub name: String,
    pub description: String,
    pub duration: i32,
    pub difficulty: Difficulty,
    pub user_id: Uuid,
    pub exercises: Vec<Option<i64>>,
}
