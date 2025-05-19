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
    schema::{exercise_logs, workout_logs},
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
    Insertable,
    Associations,
)]
#[diesel(primary_key(log_id))]
#[diesel(table_name = exercise_logs)]
#[diesel(belongs_to(User))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ExerciseLog {
    pub log_id: i32,
    pub user_id: Uuid,
    pub exercise_id: i64,
    pub sets_completed: i32,
    pub reps_completed: i32,
    pub date: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl ExerciseLog {
    pub async fn find(user: Uuid, id: i32, connection: &mut Conn) -> Result<ExerciseLog, Error> {
        exercise_logs::table
            .filter(exercise_logs::dsl::user_id.eq(user))
            .filter(exercise_logs::log_id.eq(id))
            .select(ExerciseLog::as_select())
            .first(connection)
            .await
            .map_err(|error| {
                tracing::error!("[DB]: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn all(connection: &mut Conn) -> Result<Vec<ExerciseLog>, Error> {
        exercise_logs::table
            .select(ExerciseLog::as_select())
            .load(connection)
            .await
            .map_err(|error| {
                tracing::error!("[DB]: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn user_all(user: Uuid, connection: &mut Conn) -> Result<Vec<ExerciseLog>, Error> {
        exercise_logs::table
            .filter(exercise_logs::dsl::user_id.eq(user))
            .select(ExerciseLog::as_select())
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
        data: UpdateExerciseLog,
        connection: &mut Conn,
    ) -> Result<ExerciseLog, Error> {
        diesel::update(exercise_logs::table)
            .filter(exercise_logs::user_id.eq(user))
            .filter(exercise_logs::log_id.eq(id))
            .set(&data)
            .get_result(connection)
            .await
            .map_err(|error| {
                tracing::error!("[DB]: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn delete(user: Uuid, id: i32, connection: &mut Conn) -> Result<usize, Error> {
        diesel::delete(exercise_logs::table)
            .filter(exercise_logs::user_id.eq(user))
            .filter(exercise_logs::log_id.eq(id))
            .execute(connection)
            .await
            .map_err(|error| {
                tracing::error!("[DB]: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn create(data: NewExerciseLog, connection: &mut Conn) -> Result<ExerciseLog, Error> {
        diesel::insert_into(exercise_logs::table)
            .values(&data)
            .get_result::<ExerciseLog>(connection)
            .await
            .map_err(|error| {
                tracing::error!("[DB]: {:?}", error);
                Error::Database(error.to_string())
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
#[diesel(primary_key(log_id))]
#[diesel(table_name = workout_logs)]
#[diesel(belongs_to(User))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct WorkoutLog {
    pub log_id: i32,
    pub user_id: Uuid,
    pub workout_id: Uuid,
    pub date: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl WorkoutLog {
    pub async fn find(user: Uuid, id: i32, connection: &mut Conn) -> Result<WorkoutLog, Error> {
        workout_logs::table
            .filter(workout_logs::user_id.eq(user))
            .filter(workout_logs::log_id.eq(id))
            .select(WorkoutLog::as_select())
            .first(connection)
            .await
            .map_err(|error| {
                tracing::error!("[DB]: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn all(connection: &mut Conn) -> Result<Vec<WorkoutLog>, Error> {
        workout_logs::table
            .select(WorkoutLog::as_select())
            .load(connection)
            .await
            .map_err(|error| {
                tracing::error!("[DB]: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn user_all(user: Uuid, connection: &mut Conn) -> Result<Vec<WorkoutLog>, Error> {
        workout_logs::table
            .filter(workout_logs::user_id.eq(user))
            .select(WorkoutLog::as_select())
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
        data: UpdateWorkoutLog,
        connection: &mut Conn,
    ) -> Result<WorkoutLog, Error> {
        diesel::update(workout_logs::table)
            .filter(workout_logs::user_id.eq(user))
            .filter(workout_logs::log_id.eq(id))
            .set(&data)
            .get_result(connection)
            .await
            .map_err(|error| {
                tracing::error!("[DB]: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn delete(user: Uuid, id: i32, connection: &mut Conn) -> Result<usize, Error> {
        diesel::delete(workout_logs::table)
            .filter(workout_logs::user_id.eq(user))
            .filter(workout_logs::log_id.eq(id))
            .execute(connection)
            .await
            .map_err(|error| {
                tracing::error!("[DB]: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn create(data: NewWorkoutLog, connection: &mut Conn) -> Result<WorkoutLog, Error> {
        diesel::insert_into(workout_logs::table)
            .values(&data)
            .get_result::<WorkoutLog>(connection)
            .await
            .map_err(|error| {
                tracing::error!("[DB]: {:?}", error);
                Error::Database(error.to_string())
            })
    }
}

#[derive(AsChangeset)]
#[diesel(table_name = workout_logs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateWorkoutLog {
    pub date: Option<NaiveDateTime>,
}

#[derive(AsChangeset)]
#[diesel(table_name = workout_logs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
pub struct NewWorkoutLog {
    pub user_id: Uuid,
    pub workout_id: Uuid,
}
