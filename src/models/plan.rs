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
    models::{profile::FitnessGoal, user::User, workout::Difficulty},
    responders::ProteinError,
    schema::{
        workout_plans,
        workout_plans::dsl::{user_id, plan_id},
    },
};

#[derive(DbEnum, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[ExistingTypePath = "crate::schema::sql_types::Workoutinterval"]
pub enum WorkoutInterval {
    Daily,
    Weekly,
    Monthly,
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
#[diesel(primary_key(plan_id))]
#[diesel(table_name = workout_plans)]
#[diesel(belongs_to(User))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct WorkoutPlan {
    pub plan_id: Uuid,
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

impl WorkoutPlan {
    pub async fn find(
        user: &User,
        plan: Uuid,
        connection: &mut DatabaseConnection,
    ) -> Result<WorkoutPlan, ProteinError> {
        WorkoutPlan::belonging_to(user)
            .select(WorkoutPlan::as_select())
            .filter(plan_id.eq(plan_id))
            .first(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                ProteinError::Database(error.to_string())
            })
    }

    pub async fn all(
        connection: &mut DatabaseConnection,
    ) -> Result<Vec<WorkoutPlan>, ProteinError> {
        workout_plans::table
            .select(WorkoutPlan::as_select())
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
    ) -> Result<Vec<WorkoutPlan>, ProteinError> {
        workout_plans::table
            .filter(user_id.eq(user))
            .select(WorkoutPlan::as_select())
            .load(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                ProteinError::Database(error.to_string())
            })
    }

    pub async fn update(
        user: Uuid,
        plan: Uuid,
        data: UpdateWorkoutPlan,
        connection: &mut DatabaseConnection,
    ) -> Result<WorkoutPlan, ProteinError> {
        diesel::update(workout_plans::table)
            .filter(user_id.eq(user))
            .filter(plan_id.eq(plan))
            .set(&data)
            .get_result::<WorkoutPlan>(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                ProteinError::Database(error.to_string())
            })
    }

    pub async fn delete(
        user: Uuid,
        plan: Uuid,
        connection: &mut DatabaseConnection,
    ) -> Result<usize, ProteinError> {
        diesel::delete(workout_plans::table)
            .filter(user_id.eq(user))
            .filter(plan_id.eq(plan))
            .execute(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                ProteinError::Database(error.to_string())
            })
    }

    pub async fn create(
        data: NewWorkoutPlan,
        connection: &mut DatabaseConnection,
    ) -> Result<WorkoutPlan, ProteinError> {
        diesel::insert_into(workout_plans::table)
            .values(&data)
            .get_result::<WorkoutPlan>(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                ProteinError::Database(error.to_string())
            })
    }
}

#[derive(AsChangeset)]
#[diesel(table_name = workout_plans)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateWorkoutPlan {
    pub name: Option<String>,
    pub description: Option<String>,
    pub workouts: Option<Vec<Option<Uuid>>>,
    pub start_time: Option<NaiveDateTime>,
    pub repeats: Option<WorkoutInterval>,
    pub goal: Option<FitnessGoal>,
    pub difficulty: Option<Difficulty>,
    pub is_public: Option<bool>,
}

#[derive(AsChangeset)]
#[diesel(table_name = workout_plans)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
pub struct NewWorkoutPlan {
    pub user_id: Uuid,
    pub name: String,
    pub description: String,
    pub workouts: Vec<Option<Uuid>>,
    pub start_time: NaiveDateTime,
    pub repeats: WorkoutInterval,
    pub goal: FitnessGoal,
    pub difficulty: Difficulty,
    pub is_public: bool,
}
