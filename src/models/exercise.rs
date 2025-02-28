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
use validator::Validate;
use diesel_async::RunQueryDsl;
use diesel_derive_enum::DbEnum;
use rocket::serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

use crate::{
    db::DBConnection,
    errors::Error,
    models::workout::Difficulty,
    schema::{exercises, exercises::dsl::exercise_id},
};

// Exercise Model
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
#[diesel(table_name = exercises)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Exercise {
    pub exercise_id: i64,
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

impl Exercise {
    pub async fn find(id: i64, connection: &mut DBConnection) -> Result<Exercise, Error> {
        exercises::table
            .find(id)
            .select(Exercise::as_select())
            .first(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn search(
        connection: &mut DBConnection,
        data: SearchExercise,
    ) -> Result<Vec<Exercise>, Error> {
        let mut query = exercises::table.into_boxed();

        if let Some(name) = data.name {
            query = query.filter(exercises::name.eq(name));
        }

        if let Some(equipment) = data.equipment {
            query = query.filter(exercises::equipment.eq(equipment));
        }

        if let Some(difficulty) = data.difficulty {
            query = query.filter(exercises::difficulty.eq(difficulty));
        }

        if let Some(muscle_group) = data.muscle_group {
            query = query.filter(exercises::muscle_group.eq(muscle_group));
        }

        query.load::<Exercise>(connection).await.map_err(|error| {
            tracing::error!("[!] PostgreSQL Error: {:?}", error);
            Error::Database(error.to_string())
        })
    }

    pub async fn all(connection: &mut DBConnection) -> Result<Vec<Exercise>, Error> {
        exercises::table
            .select(Exercise::as_select())
            .load(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn delete(exercise: i64, connection: &mut DBConnection) -> Result<usize, Error> {
        diesel::delete(exercises::table)
            .filter(exercise_id.eq(exercise))
            .execute(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn update(
        exercise: i64,
        data: UpdateExercise,
        connection: &mut DBConnection,
    ) -> Result<Exercise, Error> {
        diesel::update(exercises::table)
            .filter(exercise_id.eq(exercise))
            .set(&data)
            .get_result::<Exercise>(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn create(
        connection: &mut DBConnection,
        data: NewExercise,
    ) -> Result<Exercise, Error> {
        diesel::insert_into(exercises::table)
            .values(&data)
            .get_result(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                Error::Database(error.to_string())
            })
    }
}

#[derive(AsChangeset)]
#[diesel(table_name = exercises)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Serialize, Deserialize, Clone, Validate)]
pub struct UpdateExercise {
    pub name: Option<String>,
    pub description: Option<String>,
    pub instructions: Option<String>,
    pub equipment: Option<Equipment>,
    pub difficulty: Option<Difficulty>,
    pub muscle_group: Option<MuscleGroup>,
    pub sets: Option<i32>,
    pub reps: Option<i32>,
    pub rest_time: Option<i32>,
    pub exercise_type: Option<ExerciseType>,
    #[validate(url)]
    pub image_url: Option<String>,
    #[validate(url)]
    pub video_url: Option<String>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize, Validate)]
#[diesel(table_name = exercises)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewExercise {
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
    #[validate(url)]
    pub image_url: String,
    #[validate(url)]
    pub video_url: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SearchExercise {
    pub name: Option<String>,
    pub equipment: Option<Equipment>,
    pub difficulty: Option<Difficulty>,
    pub muscle_group: Option<MuscleGroup>,
    pub exercise_type: Option<ExerciseType>,
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
