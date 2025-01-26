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
    schema::{
        calorie_logs, calorie_logs::dsl::*, protein_logs, protein_logs::dsl::*, sleep_logs,
        sleep_logs::dsl::*, water_logs, water_logs::dsl::*,
    },
};

// Nutrition Calorie Logs
#[derive(
    Clone, Debug, Eq, PartialEq, Queryable, Selectable, Serialize, Deserialize, Identifiable,
)]
#[diesel(table_name = calorie_logs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CalorieLog {
    pub id: i32,
    pub user_id: Uuid,
    pub date: NaiveDateTime,
    pub amount: i32,
    pub updated_at: NaiveDateTime,
}

// Nutrition Protein Logs
#[derive(
    Clone, Debug, Eq, PartialEq, Queryable, Selectable, Serialize, Deserialize, Identifiable,
)]
#[diesel(table_name = protein_logs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ProteinLog {
    id: i32,
    user_id: Uuid,
    date: NaiveDateTime,
    amount: i32,
    updated_at: NaiveDateTime,
}

// Nutrition Sleep Logs
#[derive(
    Clone, Debug, Eq, PartialEq, Queryable, Selectable, Serialize, Deserialize, Identifiable,
)]
#[diesel(table_name = sleep_logs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SleepLog {
    id: i32,
    user_id: Uuid,
    beginning: NaiveDateTime,
    end: NaiveDateTime,
    updated_at: NaiveDateTime,
    amount: i32,
}

// Nutrition Water Logs
#[derive(
    Clone, Debug, Eq, PartialEq, Queryable, Selectable, Serialize, Deserialize, Identifiable,
)]
#[diesel(table_name = water_logs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct WaterLog {
    id: i32,
    user_id: Uuid,
    amount: i32,
    updated_at: NaiveDateTime,
}
