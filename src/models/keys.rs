/*
______          _       _
| ___ \        | |     (_)
| |_/ / __ ___ | |_ ___ _ _ __
|  __/ '__/ _ \| __/ _ \ | '_ \
| |  | | | (_) | ||  __/ | | | |
\_|  |_|  \___/ \__\___|_|_| |_|

        Made with ❤️
*/

// Uuid
use uuid::Uuid;

// Diesel
use diesel::prelude::*;

// Diesel Async
use diesel_async::RunQueryDsl;

// Serde Serialization
use rocket::serde::{Deserialize, Serialize};

// Time
use chrono::NaiveDateTime;

// Users
use crate::{
    schema::api_keys, schema::api_keys::dsl::*, models::user::User, db::DatabaseConnection,
    responders::ProteinError,
};

#[derive(
    Serialize,
    Deserialize,
    Queryable,
    Identifiable,
    Associations,
    Selectable,
    Insertable,
    Debug,
    Clone,
    PartialEq,
)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = api_keys)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(User))]
pub struct APIKey {
    pub id: i32,
    pub user_id: Uuid,
    pub api_key: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub is_developer_key: bool,
}

impl APIKey {
    pub async fn get(
        user: &User,
        connection: &mut DatabaseConnection,
    ) -> Result<APIKey, ProteinError> {
        APIKey::belonging_to(user)
            .select(APIKey::as_select())
            .first(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                ProteinError::Database(error.to_string())
            })
    }

    pub async fn generate(
        user: &User,
        connection: &mut DatabaseConnection,
    ) -> Result<APIKey, ProteinError> {
        diesel::insert_into(api_keys::table)
            .values(user_id.eq(user.id))
            .get_result(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                ProteinError::Database(error.to_string())
            })
    }

    pub async fn find_by_key(
        key: Uuid,
        mut connection: DatabaseConnection,
    ) -> Result<APIKey, ProteinError> {
        api_keys::table
            .filter(api_key.eq(key))
            .select(APIKey::as_select())
            .first(&mut connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                ProteinError::Database(error.to_string())
            })
    }
}
