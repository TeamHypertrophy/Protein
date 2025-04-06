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
    db::DBConnection,
    errors::Error,
    models::{profile::FitnessGoal, user::User, workout::Difficulty},
    schema::{workout_plan_logs, workout_plans},
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
        connection: &mut DBConnection,
    ) -> Result<WorkoutPlan, Error> {
        WorkoutPlan::belonging_to(user)
            .select(WorkoutPlan::as_select())
            .filter(workout_plans::plan_id.eq(plan))
            .first(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn all(connection: &mut DBConnection) -> Result<Vec<WorkoutPlan>, Error> {
        workout_plans::table
            .select(WorkoutPlan::as_select())
            .load(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn user_all(
        user: Uuid,
        connection: &mut DBConnection,
    ) -> Result<Vec<WorkoutPlan>, Error> {
        workout_plans::table
            .filter(workout_plans::user_id.eq(user))
            .select(WorkoutPlan::as_select())
            .load(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn update(
        user: Uuid,
        plan: Uuid,
        data: UpdateWorkoutPlan,
        connection: &mut DBConnection,
    ) -> Result<WorkoutPlan, Error> {
        diesel::update(workout_plans::table)
            .filter(workout_plans::user_id.eq(user))
            .filter(workout_plans::plan_id.eq(plan))
            .set(&data)
            .get_result::<WorkoutPlan>(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn delete(
        user: Uuid,
        plan: Uuid,
        connection: &mut DBConnection,
    ) -> Result<usize, Error> {
        diesel::delete(workout_plans::table)
            .filter(workout_plans::user_id.eq(user))
            .filter(workout_plans::plan_id.eq(plan))
            .execute(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn create(
        data: NewWorkoutPlan,
        connection: &mut DBConnection,
    ) -> Result<WorkoutPlan, Error> {
        diesel::insert_into(workout_plans::table)
            .values(&data)
            .get_result::<WorkoutPlan>(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                Error::Database(error.to_string())
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

#[derive(AsChangeset, Insertable)]
#[diesel(table_name = workout_plans)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[diesel(table_name = workout_plan_logs)]
#[diesel(belongs_to(User))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct WorkoutPlanLog {
    pub log_id: i32,
    pub user_id: Uuid,
    pub plan_id: Uuid,
    pub date: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl WorkoutPlanLog {
    pub async fn find(
        user: &User,
        plan: Uuid,
        connection: &mut DBConnection,
    ) -> Result<WorkoutPlanLog, Error> {
        WorkoutPlanLog::belonging_to(user)
            .select(WorkoutPlanLog::as_select())
            .filter(workout_plan_logs::plan_id.eq(plan))
            .first(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn all(connection: &mut DBConnection) -> Result<Vec<WorkoutPlanLog>, Error> {
        workout_plan_logs::table
            .select(WorkoutPlanLog::as_select())
            .load(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn user_all(
        user: Uuid,
        connection: &mut DBConnection,
    ) -> Result<Vec<WorkoutPlanLog>, Error> {
        workout_plan_logs::table
            .filter(workout_plan_logs::user_id.eq(user))
            .select(WorkoutPlanLog::as_select())
            .load(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn update(
        user: Uuid,
        plan: Uuid,
        data: UpdateWorkoutPlanLog,
        connection: &mut DBConnection,
    ) -> Result<WorkoutPlanLog, Error> {
        diesel::update(workout_plan_logs::table)
            .filter(workout_plan_logs::user_id.eq(user))
            .filter(workout_plan_logs::plan_id.eq(plan))
            .set(&data)
            .get_result::<WorkoutPlanLog>(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn delete(
        user: Uuid,
        plan: Uuid,
        connection: &mut DBConnection,
    ) -> Result<usize, Error> {
        diesel::delete(workout_plan_logs::table)
            .filter(workout_plan_logs::user_id.eq(user))
            .filter(workout_plan_logs::plan_id.eq(plan))
            .execute(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn create(
        data: NewWorkoutPlanLog,
        connection: &mut DBConnection,
    ) -> Result<WorkoutPlanLog, Error> {
        diesel::insert_into(workout_plan_logs::table)
            .values(&data)
            .get_result::<WorkoutPlanLog>(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                Error::Database(error.to_string())
            })
    }
}

#[derive(AsChangeset)]
#[diesel(table_name = workout_plan_logs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
pub struct UpdateWorkoutPlanLog {
    pub date: Option<NaiveDateTime>,
}

#[derive(AsChangeset, Insertable)]
#[diesel(table_name = workout_plan_logs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewWorkoutPlanLog {
    pub user_id: Uuid,
    pub plan_id: Uuid,
    pub date: NaiveDateTime,
}
