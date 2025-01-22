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

use crate::{
    db::DatabaseConnection,
    schema::{profile, profile::dsl::*},
};

// Profile Model
#[derive(Clone, Debug, PartialEq, Queryable, Selectable, Serialize, Deserialize, Identifiable)]
#[diesel(table_name = profile)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Profile {
    pub id: i32,
    pub user_id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub age: i32,
    pub gender: Gender,
    pub weight: f64,
    pub height: f64,
    pub preferred_weight_unit: PreferredWeight,
    pub preferred_height_unit: PreferredHeight,
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
