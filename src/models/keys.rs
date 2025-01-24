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
    models::user::User,
    responders::ProteinError,
    schema::{api_keys, api_keys::dsl::*},
};

// APIKey Model
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
        is_dev: bool,
        connection: &mut DatabaseConnection,
    ) -> Result<APIKey, ProteinError> {
        diesel::insert_into(api_keys::table)
            .values((user_id.eq(user.id), is_developer_key.eq(is_dev)))
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

    pub async fn verify_api_key(
        key: Uuid,
        mut connection: DatabaseConnection,
    ) -> Result<bool, ProteinError> {
        todo!()
    }
}
