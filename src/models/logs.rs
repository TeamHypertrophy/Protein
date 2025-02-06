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
    models::{user::User, workout::Difficulty},
    responders::ProteinError,
    schema::{
        exercise_logs, exercise_logs::dsl::id as exercise_dsl_id, workout_logs,
        workout_logs::dsl::id as workout_dsl_id,
    },
};

// Exercise Log
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
#[diesel(table_name = exercise_logs)]
#[diesel(belongs_to(User))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ExerciseLog {
    pub id: i32,
    pub user_id: Uuid,
    pub exercise_id: i64,
    pub sets_completed: i32,
    pub reps_completed: i32,
    pub date: NaiveDateTime,
    pub updated_at: NaiveDateTime,
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
    Associations,
)]
#[diesel(table_name = workout_logs)]
#[diesel(belongs_to(User))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct WorkoutLog {
    pub id: i32,
    pub user_id: Uuid,
    pub workout_id: Uuid,
    pub date: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
