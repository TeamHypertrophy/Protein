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

impl ExerciseLog {
    pub async fn find(
        user: Uuid,
        log_id: i32,
        connection: &mut DatabaseConnection,
    ) -> Result<ExerciseLog, ProteinError> {
        exercise_logs::table
            .filter(exercise_logs::dsl::user_id.eq(user))
            .filter(exercise_dsl_id.eq(log_id))
            .select(ExerciseLog::as_select())
            .first(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                ProteinError::Database(error.to_string())
            })
    }

    pub async fn all(
        connection: &mut DatabaseConnection,
    ) -> Result<Vec<ExerciseLog>, ProteinError> {
        exercise_logs::table
            .select(ExerciseLog::as_select())
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
    ) -> Result<Vec<ExerciseLog>, ProteinError> {
        exercise_logs::table
            .filter(exercise_logs::dsl::user_id.eq(user))
            .select(ExerciseLog::as_select())
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
        data: UpdateExerciseLog,
        connection: &mut DatabaseConnection,
    ) -> Result<ExerciseLog, ProteinError> {
        diesel::update(exercise_logs::table)
            .filter(exercise_logs::dsl::user_id.eq(user))
            .filter(exercise_dsl_id.eq(log_id))
            .set(&data)
            .get_result(connection)
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
        diesel::delete(exercise_logs::table)
            .filter(exercise_logs::dsl::user_id.eq(user))
            .filter(exercise_dsl_id.eq(log_id))
            .execute(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                ProteinError::Database(error.to_string())
            })
    }
}

#[derive(AsChangeset)]
#[diesel(table_name = exercise_logs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateExerciseLog {
    pub sets_completed: Option<i32>,
    pub reps_completed: Option<i32>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(AsChangeset)]
#[diesel(table_name = exercise_logs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
pub struct NewExerciseLog {
    pub user_id: Uuid,
    pub exercise_id: i64,
    pub sets_completed: i32,
    pub reps_completed: i32,
    pub updated_at: NaiveDateTime,
}

impl NewExerciseLog {
    pub async fn create(
        data: NewExerciseLog,
        connection: &mut DatabaseConnection,
    ) -> Result<ExerciseLog, ProteinError> {
        diesel::insert_into(exercise_logs::table)
            .values(&data)
            .get_result::<ExerciseLog>(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                ProteinError::Database(error.to_string())
            })
    }
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

impl WorkoutLog {
    pub async fn find(
        user: Uuid,
        log_id: Uuid,
        connection: &mut DatabaseConnection,
    ) -> Result<WorkoutLog, ProteinError> {
        workout_logs::table
            .filter(workout_logs::dsl::user_id.eq(user))
            .filter(workout_logs::dsl::workout_id.eq(log_id))
            .select(WorkoutLog::as_select())
            .first(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                ProteinError::Database(error.to_string())
            })
    }

    pub async fn all(connection: &mut DatabaseConnection) -> Result<Vec<WorkoutLog>, ProteinError> {
        workout_logs::table
            .select(WorkoutLog::as_select())
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
    ) -> Result<Vec<WorkoutLog>, ProteinError> {
        workout_logs::table
            .filter(workout_logs::dsl::user_id.eq(user))
            .select(WorkoutLog::as_select())
            .load(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                ProteinError::Database(error.to_string())
            })
    }

    pub async fn update(
        user: Uuid,
        log_id: Uuid,
        data: UpdateWorkoutLog,
        connection: &mut DatabaseConnection,
    ) -> Result<WorkoutLog, ProteinError> {
        diesel::update(workout_logs::table)
            .filter(workout_logs::dsl::user_id.eq(user))
            .filter(workout_logs::dsl::workout_id.eq(log_id))
            .set(&data)
            .get_result(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                ProteinError::Database(error.to_string())
            })
    }

    pub async fn delete(
        user: Uuid,
        log_id: Uuid,
        connection: &mut DatabaseConnection,
    ) -> Result<usize, ProteinError> {
        diesel::delete(workout_logs::table)
            .filter(workout_logs::dsl::user_id.eq(user))
            .filter(workout_logs::dsl::workout_id.eq(log_id))
            .execute(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                ProteinError::Database(error.to_string())
            })
    }
}

#[derive(AsChangeset)]
#[diesel(table_name = workout_logs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateWorkoutLog {
    pub date: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(AsChangeset)]
#[diesel(table_name = workout_logs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
pub struct NewWorkoutLog {
    pub user_id: Uuid,
    pub workout_id: Uuid,
}

impl NewWorkoutLog {
    pub async fn create(
        data: NewWorkoutLog,
        connection: &mut DatabaseConnection,
    ) -> Result<WorkoutLog, ProteinError> {
        diesel::insert_into(workout_logs::table)
            .values(&data)
            .get_result::<WorkoutLog>(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                ProteinError::Database(error.to_string())
            })
    }
}
