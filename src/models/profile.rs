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

use crate::{db::DBConnection, errors::Error, models::user::User, schema::profiles};

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
    Insertable,
    Associations,
)]
#[diesel(primary_key(profile_id))]
#[diesel(table_name = profiles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(User))]
pub struct Profile {
    pub profile_id: i32,
    pub user_id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub age: i32,
    pub weight: f64,
    pub height: f64,
    pub gender: Gender,
    pub preferred_weight_unit: PreferredWeight,
    pub preferred_height_unit: PreferredHeight,
    pub public: bool,
    pub bio: String,
    pub streak: i32,
    pub avatar_url: String,
    pub activity_level: ActivityLevel,
    pub fitness_goal: FitnessGoal,
    pub diet: Diet,
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

#[derive(DbEnum, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[ExistingTypePath = "crate::schema::sql_types::Activitylevel"]
pub enum ActivityLevel {
    Light,
    Moderate,
    Very,
    Extremely,
}

#[derive(DbEnum, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[ExistingTypePath = "crate::schema::sql_types::Diet"]
pub enum Diet {
    Vegetarian,
    Vegan,
    Keto,
    Anything,
}

impl Profile {
    pub async fn find(user: &User, connection: &mut DBConnection) -> Result<Profile, Error> {
        Profile::belonging_to(user)
            .select(Profile::as_select())
            .first(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn all(connection: &mut DBConnection) -> Result<Vec<Profile>, Error> {
        profiles::table
            .select(Profile::as_select())
            .load(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn update(
        user: Uuid,
        data: UpdateProfile,
        connection: &mut DBConnection,
    ) -> Result<Profile, Error> {
        diesel::update(profiles::table)
            .filter(profiles::user_id.eq(user))
            .set(&data)
            .get_result::<Profile>(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn upload_avatar(
        user: Uuid,
        data: String,
        connection: &mut DBConnection,
    ) -> Result<Profile, Error> {
        diesel::update(profiles::table)
            .filter(profiles::user_id.eq(user))
            .set(profiles::avatar_url.eq(data))
            .get_result::<Profile>(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn delete(user: Uuid, connection: &mut DBConnection) -> Result<usize, Error> {
        diesel::delete(profiles::table)
            .filter(profiles::user_id.eq(user))
            .execute(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn create(data: NewProfile, connection: &mut DBConnection) -> Result<Profile, Error> {
        diesel::insert_into(profiles::table)
            .values(&data)
            .get_result::<Profile>(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn leaderboard(connection: &mut DBConnection) -> Result<Vec<Profile>, Error> {
        profiles::table
            .select(Profile::as_select())
            .order_by(profiles::streak.desc())
            .limit(100)
            .load(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                Error::Database(error.to_string())
            })
    }
}

#[derive(AsChangeset)]
#[diesel(table_name = profiles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateProfile {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    #[validate(range(min = 13, max = 100))]
    pub age: Option<i32>,
    pub weight: Option<f64>,
    pub height: Option<f64>,
    pub gender: Option<Gender>,
    pub preferred_weight_unit: Option<PreferredWeight>,
    pub preferred_height_unit: Option<PreferredHeight>,
    pub public: Option<bool>,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub streak: Option<i32>,
    pub activity_level: Option<ActivityLevel>,
    pub fitness_goal: Option<FitnessGoal>,
    pub diet: Option<Diet>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize, Validate)]
#[diesel(table_name = profiles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewProfile {
    pub user_id: Uuid,
    pub first_name: String,
    pub last_name: String,
    #[validate(range(min = 13, max = 100))]
    pub age: i32,
    pub gender: Gender,
}
