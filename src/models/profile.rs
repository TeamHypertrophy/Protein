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
use validator::Validate;
use diesel_async::RunQueryDsl;
use chrono::NaiveDateTime;
use rocket::serde::{Deserialize, Serialize};

use crate::{
    db::DatabaseConnection,
    models::user::User,
    responders::ProteinError,
    schema::{profiles, profiles::dsl::user_id as dsl_user_id},
};

// Profile Model
#[derive(
    Clone,
    Debug,
    PartialEq,
    Queryable,
    Selectable,
    QueryableByName,
    Serialize,
    Deserialize,
    Identifiable,
    Associations,
)]
#[diesel(table_name = profiles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(User))]
pub struct Profile {
    pub id: i32,
    pub user_id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub age: i32,
    pub weight: f64,
    pub height: f64,
    pub gender: Gender,
    pub preferred_weight_unit: PreferredWeight,
    pub preferred_height_unit: PreferredHeight,
    pub public: bool,
    pub bio: String,
    pub avatar_url: String,
    pub fitness_goal: FitnessGoal,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(DbEnum, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[ExistingTypePath = "crate::schema::sql_types::Gender"]
pub enum Gender {
    Male,
    Female,
}

#[derive(DbEnum, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[ExistingTypePath = "crate::schema::sql_types::Preferredweight"]
pub enum PreferredWeight {
    Kg,
    Lbs,
}

#[derive(DbEnum, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[ExistingTypePath = "crate::schema::sql_types::Preferredheight"]
pub enum PreferredHeight {
    Cm,
    In,
}

#[derive(DbEnum, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[ExistingTypePath = "crate::schema::sql_types::Fitnessgoal"]
pub enum FitnessGoal {
    WeightLoss,
    MuscleGain,
    Maintenance,
    Endurance,
    Strength,
}

impl Profile {
    pub async fn find(
        user: &User,
        connection: &mut DatabaseConnection,
    ) -> Result<Profile, ProteinError> {
        Profile::belonging_to(user)
            .select(Profile::as_select())
            .first(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                ProteinError::Database(error.to_string())
            })
    }

    pub async fn find_by_email(
        address: String,
        connection: &mut DatabaseConnection,
    ) -> Result<Profile, ProteinError> {
        profiles::table
            .filter(profiles::email.eq(address))
            .select(Profile::as_select())
            .first(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                ProteinError::Database(error.to_string())
            })
    }

    pub async fn all(connection: &mut DatabaseConnection) -> Result<Vec<Profile>, ProteinError> {
        profiles::table
            .select(Profile::as_select())
            .load(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                ProteinError::Database(error.to_string())
            })
    }

    pub async fn update(
        user: Uuid,
        profile: UpdateProfile,
        connection: &mut DatabaseConnection,
    ) -> Result<Profile, ProteinError> {
        diesel::update(profiles::table)
            .filter(dsl_user_id.eq(user))
            .set(&profile)
            .get_result::<Profile>(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                ProteinError::Database(error.to_string())
            })
    }

    pub async fn delete(
        user: Uuid,
        connection: &mut DatabaseConnection,
    ) -> Result<usize, ProteinError> {
        diesel::delete(profiles::table)
            .filter(dsl_user_id.eq(user))
            .execute(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                ProteinError::Database(error.to_string())
            })
    }
}

#[derive(AsChangeset)]
#[diesel(table_name = profiles)]
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateProfile {
    pub first_name: String,
    pub last_name: String,
    #[validate(email)]
    pub email: String,
    #[validate(range(min = 13, max = 100))]
    pub age: i32,
    pub weight: f64,
    pub height: f64,
    pub gender: Gender,
    pub preferred_weight_unit: PreferredWeight,
    pub preferred_height_unit: PreferredHeight,
    pub public: bool,
    pub bio: String,
    pub avatar_url: String,
    pub fitness_goal: FitnessGoal,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize, Validate)]
#[diesel(table_name = profiles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewProfile {
    pub user_id: Uuid,
    pub first_name: String,
    pub last_name: String,
    #[validate(email)]
    pub email: String,
    #[validate(range(min = 13, max = 100))]
    pub age: i32,
    pub gender: Gender,
}

impl NewProfile {
    pub async fn create(
        profile: NewProfile,
        connection: &mut DatabaseConnection,
    ) -> Result<Profile, ProteinError> {
        diesel::insert_into(profiles::table)
            .values(&profile)
            .get_result::<Profile>(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                ProteinError::Database(error.to_string())
            })
    }
}

#[derive(Serialize, Deserialize)]
pub struct ForgotPassword {
    pub email: String,
}
