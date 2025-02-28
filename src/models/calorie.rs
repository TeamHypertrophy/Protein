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
    db::DBConnection,
    errors::Error,
    models::user::User,
    schema::{
        calorie_logs,
        calorie_logs::dsl::{log_id, user_id},
    },
};

// Nutrition Calorie Logs
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
#[diesel(primary_key(log_id))]
#[diesel(table_name = calorie_logs)]
#[diesel(belongs_to(User))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CalorieLog {
    pub log_id: i32,
    pub user_id: Uuid,
    pub date: NaiveDateTime,
    pub amount: i32,
    pub updated_at: NaiveDateTime,
}

impl CalorieLog {
    pub async fn find(
        user: Uuid,
        id: i32,
        connection: &mut DBConnection,
    ) -> Result<CalorieLog, Error> {
        calorie_logs::table
            .filter(user_id.eq(user))
            .filter(log_id.eq(id))
            .select(CalorieLog::as_select())
            .first(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn all(connection: &mut DBConnection) -> Result<Vec<CalorieLog>, Error> {
        calorie_logs::table
            .select(CalorieLog::as_select())
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
    ) -> Result<Vec<CalorieLog>, Error> {
        calorie_logs::table
            .filter(user_id.eq(user))
            .select(CalorieLog::as_select())
            .load(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn update(
        user: Uuid,
        id: i32,
        data: UpdateCalorieLog,
        connection: &mut DBConnection,
    ) -> Result<CalorieLog, Error> {
        diesel::update(calorie_logs::table)
            .filter(user_id.eq(user))
            .filter(log_id.eq(id))
            .set(&data)
            .get_result::<CalorieLog>(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn delete(
        user: Uuid,
        id: i32,
        connection: &mut DBConnection,
    ) -> Result<usize, Error> {
        diesel::delete(calorie_logs::table)
            .filter(user_id.eq(user))
            .filter(log_id.eq(id))
            .execute(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn create(
        data: NewCalorieLog,
        connection: &mut DBConnection,
    ) -> Result<CalorieLog, Error> {
        diesel::insert_into(calorie_logs::table)
            .values(&data)
            .get_result::<CalorieLog>(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                Error::Database(error.to_string())
            })
    }
}

#[derive(AsChangeset)]
#[diesel(table_name = calorie_logs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCalorieLog {
    pub amount: Option<i32>,
    pub date: Option<NaiveDateTime>,
}

#[derive(AsChangeset)]
#[diesel(table_name = calorie_logs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
pub struct NewCalorieLog {
    pub user_id: Uuid,
    pub amount: i32,
}
