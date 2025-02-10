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
    schema::{sleep_logs, sleep_logs::dsl::*},
};

// Nutrition Sleep Logs
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
#[diesel(table_name = sleep_logs)]
#[diesel(belongs_to(User))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SleepLog {
    id: i32,
    user_id: Uuid,
    beginning: NaiveDateTime,
    end: NaiveDateTime,
    amount: i32,
    updated_at: NaiveDateTime,
}

impl SleepLog {
    pub async fn find(
        user: Uuid,
        log_id: i32,
        connection: &mut DatabaseConnection,
    ) -> Result<SleepLog, ProteinError> {
        sleep_logs::table
            .filter(user_id.eq(user))
            .filter(id.eq(log_id))
            .select(SleepLog::as_select())
            .first(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                ProteinError::Database(error.to_string())
            })
    }

    pub async fn all(connection: &mut DatabaseConnection) -> Result<Vec<SleepLog>, ProteinError> {
        sleep_logs::table
            .select(SleepLog::as_select())
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
    ) -> Result<Vec<SleepLog>, ProteinError> {
        sleep_logs::table
            .filter(user_id.eq(user))
            .select(SleepLog::as_select())
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
        data: UpdateSleepLog,
        connection: &mut DatabaseConnection,
    ) -> Result<SleepLog, ProteinError> {
        diesel::update(sleep_logs::table)
            .filter(user_id.eq(user))
            .filter(id.eq(log_id))
            .set(&data)
            .get_result::<SleepLog>(connection)
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
        diesel::delete(sleep_logs::table)
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
#[diesel(table_name = sleep_logs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSleepLog {
    pub beginning: Option<NaiveDateTime>,
    pub end: Option<NaiveDateTime>,
    pub amount: Option<i32>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(AsChangeset)]
#[diesel(table_name = sleep_logs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
pub struct NewSleepLog {
    pub user_id: Uuid,
    pub beginning: NaiveDateTime,
    pub end: NaiveDateTime,
    pub amount: i32,
}

impl NewSleepLog {
    pub async fn create(
        data: NewSleepLog,
        connection: &mut DatabaseConnection,
    ) -> Result<SleepLog, ProteinError> {
        diesel::insert_into(sleep_logs::table)
            .values(&data)
            .get_result::<SleepLog>(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                ProteinError::Database(error.to_string())
            })
    }
}
