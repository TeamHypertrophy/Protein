/*
______          _       _
| ___ \        | |     (_)
| |_/ / __ ___ | |_ ___ _ _ __
|  __/ '__/ _ \| __/ _ \ | '_ \
| |  | | | (_) | ||  __/ | | | |
\_|  |_|  \___/ \__\___|_|_| |_|

        Made with ❤️
*/

use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use rocket::serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use uuid::Uuid;

use crate::{
    db::Conn,
    errors::Error,
    models::{
        exercise::{Equipment, ExerciseType, MuscleGroup},
        workout::Difficulty,
    },
    schema::{custom_exercises, custom_exercises::dsl::exercise_id},
};

// Custom Exercise Model
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
)]
#[diesel(primary_key(exercise_id))]
#[diesel(table_name = custom_exercises)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CustomExercise {
    pub exercise_id: i64,
    pub user_id: Uuid,
    pub name: String,
    pub equipment: Equipment,
    pub difficulty: Difficulty,
    pub muscle_group: MuscleGroup,
    pub sets: i32,
    pub reps: i32,
    pub rest_time: i32,
    pub exercise_type: ExerciseType,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl CustomExercise {
    pub async fn find(user: Uuid, id: i64, connection: &mut Conn) -> Result<CustomExercise, Error> {
        custom_exercises::table
            .filter(exercise_id.eq(id))
            .filter(custom_exercises::user_id.eq(user))
            .select(CustomExercise::as_select())
            .first(connection)
            .await
            .map_err(|error| {
                tracing::error!("[DB]: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn search(
        connection: &mut Conn,
        data: SearchCustomExercise,
    ) -> Result<Vec<CustomExercise>, Error> {
        let mut query = custom_exercises::table.into_boxed();

        if let Some(user_id) = data.user_id {
            query = query.filter(custom_exercises::user_id.eq(user_id));
        }

        if let Some(name) = data.name {
            query = query.filter(custom_exercises::name.eq(name));
        }

        if let Some(equipment) = data.equipment {
            query = query.filter(custom_exercises::equipment.eq(equipment));
        }

        if let Some(difficulty) = data.difficulty {
            query = query.filter(custom_exercises::difficulty.eq(difficulty));
        }

        if let Some(muscle_group) = data.muscle_group {
            query = query.filter(custom_exercises::muscle_group.eq(muscle_group));
        }

        query
            .load::<CustomExercise>(connection)
            .await
            .map_err(|error| {
                tracing::error!("[DB]: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn all(connection: &mut Conn) -> Result<Vec<CustomExercise>, Error> {
        custom_exercises::table
            .select(CustomExercise::as_select())
            .load(connection)
            .await
            .map_err(|error| {
                tracing::error!("[DB]: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn user_all(user: Uuid, connection: &mut Conn) -> Result<Vec<CustomExercise>, Error> {
        custom_exercises::table
            .filter(custom_exercises::user_id.eq(user))
            .select(CustomExercise::as_select())
            .load(connection)
            .await
            .map_err(|error| {
                tracing::error!("[DB]: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn delete(user: Uuid, exercise: i64, connection: &mut Conn) -> Result<usize, Error> {
        diesel::delete(custom_exercises::table)
            .filter(exercise_id.eq(exercise))
            .filter(custom_exercises::user_id.eq(user))
            .execute(connection)
            .await
            .map_err(|error| {
                tracing::error!("[DB]: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn update(
        user: Uuid,
        exercise: i64,
        data: UpdateCustomExercise,
        connection: &mut Conn,
    ) -> Result<CustomExercise, Error> {
        diesel::update(custom_exercises::table)
            .filter(custom_exercises::user_id.eq(user))
            .filter(exercise_id.eq(exercise))
            .set(&data)
            .get_result::<CustomExercise>(connection)
            .await
            .map_err(|error| {
                tracing::error!("[DB]: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn create(
        connection: &mut Conn,
        data: NewCustomExercise,
    ) -> Result<CustomExercise, Error> {
        diesel::insert_into(custom_exercises::table)
            .values(&data)
            .get_result(connection)
            .await
            .map_err(|error| {
                tracing::error!("[DB]: {:?}", error);
                Error::Database(error.to_string())
            })
    }
}

#[derive(AsChangeset)]
#[diesel(table_name = custom_exercises)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Serialize, Deserialize, Clone)]
pub struct UpdateCustomExercise {
    pub name: Option<String>,
    pub equipment: Option<Equipment>,
    pub difficulty: Option<Difficulty>,
    pub muscle_group: Option<MuscleGroup>,
    pub sets: Option<i32>,
    pub reps: Option<i32>,
    pub rest_time: Option<i32>,
    pub exercise_type: Option<ExerciseType>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = custom_exercises)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewCustomExercise {
    pub user_id: Uuid,
    pub name: String,
    pub equipment: Equipment,
    pub difficulty: Difficulty,
    pub muscle_group: MuscleGroup,
    pub sets: i32,
    pub reps: i32,
    pub rest_time: i32,
    pub exercise_type: ExerciseType,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SearchCustomExercise {
    pub user_id: Option<Uuid>,
    pub name: Option<String>,
    pub equipment: Option<Equipment>,
    pub difficulty: Option<Difficulty>,
    pub muscle_group: Option<MuscleGroup>,
    pub exercise_type: Option<ExerciseType>,
}
