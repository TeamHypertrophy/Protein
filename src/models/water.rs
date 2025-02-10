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
    models::user::User,
    responders::ProteinError,
    schema::{water_logs, water_logs::dsl::*},
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
    Associations,
)]
#[diesel(table_name = water_logs)]
#[diesel(belongs_to(User))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct WaterLog {
    id: i32,
    user_id: Uuid,
    date: NaiveDateTime,
    amount: i32,
    updated_at: NaiveDateTime,
}

impl WaterLog {
    pub async fn find(
        user: Uuid,
        log_id: i32,
        connection: &mut DatabaseConnection,
    ) -> Result<WaterLog, ProteinError> {
        water_logs::table
            .filter(user_id.eq(user))
            .filter(id.eq(log_id))
            .select(WaterLog::as_select())
            .first(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                ProteinError::Database(error.to_string())
            })
    }

    pub async fn all(connection: &mut DatabaseConnection) -> Result<Vec<WaterLog>, ProteinError> {
        water_logs::table
            .select(WaterLog::as_select())
            .load(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                ProteinError::Database(error.to_string())
            })
    }

    pub async fn user_all(
        user: Uuid,
        connection: &mut DatabaseConnection,
    ) -> Result<Vec<WaterLog>, ProteinError> {
        water_logs::table
            .filter(user_id.eq(user))
            .select(WaterLog::as_select())
            .load(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                ProteinError::Database(error.to_string())
            })
    }

    pub async fn update(
        user: Uuid,
        log_id: i32,
        data: UpdateWaterLog,
        connection: &mut DatabaseConnection,
    ) -> Result<WaterLog, ProteinError> {
        diesel::update(water_logs::table)
            .filter(user_id.eq(user))
            .filter(id.eq(log_id))
            .set(&data)
            .get_result::<WaterLog>(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                ProteinError::Database(error.to_string())
            })
    }

    pub async fn delete(
        user: Uuid,
        log_id: i32,
        connection: &mut DatabaseConnection,
    ) -> Result<usize, ProteinError> {
        diesel::delete(water_logs::table)
            .filter(user_id.eq(user))
            .filter(id.eq(log_id))
            .execute(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                ProteinError::Database(error.to_string())
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
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(AsChangeset)]
#[diesel(table_name = water_logs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
pub struct NewWaterLog {
    pub user_id: Uuid,
    pub amount: i32,
}

impl NewWaterLog {
    pub async fn create(
        data: NewWaterLog,
        connection: &mut DatabaseConnection,
    ) -> Result<WaterLog, ProteinError> {
        diesel::insert_into(water_logs::table)
            .values(&data)
            .get_result::<WaterLog>(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                ProteinError::Database(error.to_string())
            })
    }
}
