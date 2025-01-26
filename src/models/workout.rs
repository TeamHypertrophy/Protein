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
    models::profile::FitnessGoal,
    responders::ProteinError,
    schema::{
        workout_logs, workout_logs::dsl::*, workout_plans, workout_plans::dsl::*, workouts,
        workouts::dsl::*,
    },
};

#[derive(
    Clone, Debug, Eq, PartialEq, Queryable, Selectable, Serialize, Deserialize, Identifiable,
)]
#[diesel(table_name = workouts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Workout {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub duration: i32,
    pub difficulty: Difficulty,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub user_id: Uuid,
    pub exercises: Vec<Option<i64>>,
}

#[derive(DbEnum, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[ExistingTypePath = "crate::schema::sql_types::Difficulty"]
pub enum Difficulty {
    Beginner,
    Intermediate,
    Advanced,
}

#[derive(
    Clone, Debug, Eq, PartialEq, Queryable, Selectable, Serialize, Deserialize, Identifiable,
)]
#[diesel(table_name = workout_logs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct WorkoutLog {
    pub id: i32,
    pub user_id: Uuid,
    pub workout_id: Uuid,
    pub date: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(
    Clone, Debug, Eq, PartialEq, Queryable, Selectable, Serialize, Deserialize, Identifiable,
)]
#[diesel(table_name = workout_plans)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct WorkoutPlan {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub description: String,
    pub workouts: Vec<Option<Uuid>>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub start_time: NaiveDateTime,
    pub repeats: WorkoutInterval,
    pub goal: FitnessGoal,
    pub difficulty: Difficulty,
    pub is_public: bool,
}

#[derive(DbEnum, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[ExistingTypePath = "crate::schema::sql_types::Workoutinterval"]
pub enum WorkoutInterval {
    Daily,
    Weekly,
    Monthly,
}
