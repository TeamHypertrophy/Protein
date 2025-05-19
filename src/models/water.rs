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
    db::Conn,
    errors::Error,
    models::user::User,
    schema::{
        water_logs,
        water_logs::dsl::{log_id, user_id},
    },
};

// Nutrition Water Logs
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
#[diesel(table_name = water_logs)]
#[diesel(belongs_to(User))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct WaterLog {
    pub log_id: i32,
    pub user_id: Uuid,
    pub date: NaiveDateTime,
    pub amount: i32,
    pub updated_at: NaiveDateTime,
}

impl WaterLog {
    pub async fn find(user: Uuid, id: i32, connection: &mut Conn) -> Result<WaterLog, Error> {
        water_logs::table
            .filter(user_id.eq(user))
            .filter(log_id.eq(id))
            .select(WaterLog::as_select())
            .first(connection)
            .await
            .map_err(|error| {
                tracing::error!("[DB]: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn all(connection: &mut Conn) -> Result<Vec<WaterLog>, Error> {
        water_logs::table
            .select(WaterLog::as_select())
            .load(connection)
            .await
            .map_err(|error| {
                tracing::error!("[DB]: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn user_all(user: Uuid, connection: &mut Conn) -> Result<Vec<WaterLog>, Error> {
        water_logs::table
            .filter(user_id.eq(user))
            .select(WaterLog::as_select())
            .load(connection)
            .await
            .map_err(|error| {
                tracing::error!("[DB]: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn update(
        user: Uuid,
        id: i32,
        data: UpdateWaterLog,
        connection: &mut Conn,
    ) -> Result<WaterLog, Error> {
        diesel::update(water_logs::table)
            .filter(user_id.eq(user))
            .filter(log_id.eq(id))
            .set(&data)
            .get_result::<WaterLog>(connection)
            .await
            .map_err(|error| {
                tracing::error!("[DB]: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn delete(user: Uuid, id: i32, connection: &mut Conn) -> Result<usize, Error> {
        diesel::delete(water_logs::table)
            .filter(user_id.eq(user))
            .filter(log_id.eq(id))
            .execute(connection)
            .await
            .map_err(|error| {
                tracing::error!("[DB]: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn create(data: NewWaterLog, connection: &mut Conn) -> Result<WaterLog, Error> {
        diesel::insert_into(water_logs::table)
            .values(&data)
            .get_result::<WaterLog>(connection)
            .await
            .map_err(|error| {
                tracing::error!("[DB]: {:?}", error);
                Error::Database(error.to_string())
            })
    }
}

#[derive(AsChangeset)]
#[diesel(table_name = water_logs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateWaterLog {
    pub amount: Option<i32>,
    pub date: Option<NaiveDateTime>,
}

#[derive(AsChangeset)]
#[diesel(table_name = water_logs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
pub struct NewWaterLog {
    pub user_id: Uuid,
    pub amount: i32,
}
