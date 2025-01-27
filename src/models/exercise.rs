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
use diesel_derive_enum::DbEnum;
use rocket::serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

use crate::{
    db::DatabaseConnection,
    models::{user::User, workout::Difficulty},
    responders::ProteinError,
    schema::{exercise_logs, exercise_logs::dsl::*, exercises, exercises::dsl::*},
};

// Exercise Model
#[derive(
    Clone, Debug, Eq, PartialEq, Queryable, Selectable, Serialize, Deserialize, Identifiable,
)]
#[diesel(table_name = exercises)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Exercise {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub instructions: String,
    pub equipment: Equipment,
    pub difficulty: Difficulty,
    pub muscle_group: MuscleGroup,
    pub sets: i32,
    pub reps: i32,
    pub rest_time: i32,
    pub exercise_type: ExerciseType,
    pub image_url: String,
    pub video_url: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(DbEnum, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[ExistingTypePath = "crate::schema::sql_types::Equipment"]
pub enum Equipment {
    Barbell,
    Dumbbell,
    Kettlebell,
    Machine,
    Cable,
    Bodyweight,
    ResistanceBand,
    Other,
}

#[derive(DbEnum, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[ExistingTypePath = "crate::schema::sql_types::Musclegroup"]
pub enum MuscleGroup {
    Chest,
    Shoulders,
    Biceps,
    Triceps,
    Forearms,
    UpperBack,
    LowerBack,
    Lats,
    Abs,
    Obliques,
    Quads,
    Hamstrings,
    Calves,
    Glutes,
}

#[derive(DbEnum, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[ExistingTypePath = "crate::schema::sql_types::Exercisetype"]
pub enum ExerciseType {
    Strength,
    Cardio,
    Flexibility,
    Plyometric,
    Bodyweight,
}

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
