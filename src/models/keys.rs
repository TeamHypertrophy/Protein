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
use diesel_derive_enum::DbEnum;
use rocket::serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::NaiveDateTime;

use crate::{
    constants::API_QUOTA_LIMIT,
    db::DatabaseConnection,
    models::user::{Role, User},
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
    pub expires_at: NaiveDateTime,
    pub is_developer_key: bool,
    pub revoked_reason: String,
    pub status: Status,
    pub quota: i32,
}

#[derive(DbEnum, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[ExistingTypePath = "crate::schema::sql_types::Status"]
pub enum Status {
    Active,
    Revoked,
    Expired,
}

#[derive(AsChangeset, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = api_keys)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UpdateAPIKey {
    pub updated_at: Option<NaiveDateTime>,
    pub expires_at: Option<NaiveDateTime>,
    pub is_developer_key: Option<bool>,
    pub revoked_reason: Option<String>,
    pub status: Option<Status>,
    pub quota: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RevokeKey {
    pub revoked_reason: String,
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

    pub async fn all(connection: &mut DatabaseConnection) -> Result<Vec<APIKey>, ProteinError> {
        api_keys::table
            .select(APIKey::as_select())
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
    ) -> Result<Vec<APIKey>, ProteinError> {
        api_keys::table
            .filter(user_id.eq(user))
            .select(APIKey::as_select())
            .load(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                ProteinError::Database(error.to_string())
            })
    }

    pub async fn revoke(
        key: Uuid,
        data: RevokeKey,
        connection: &mut DatabaseConnection,
    ) -> Result<APIKey, ProteinError> {
        diesel::update(api_keys::table)
            .filter(api_key.eq(key))
            .set((
                revoked_reason.eq(data.revoked_reason),
                status.eq(Status::Revoked),
            ))
            .get_result(connection)
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

    pub async fn update(
        key: Uuid,
        data: UpdateAPIKey,
        connection: &mut DatabaseConnection,
    ) -> Result<APIKey, ProteinError> {
        diesel::update(api_keys::table)
            .filter(api_key.eq(key))
            .set(&data)
            .get_result::<APIKey>(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                ProteinError::Database(error.to_string())
            })
    }

    pub async fn increment(
        key: Uuid,
        original: i32,
        connection: &mut DatabaseConnection,
    ) -> Result<(), ProteinError> {
        diesel::update(api_keys::table)
            .filter(api_key.eq(key))
            .set((
                updated_at.eq(chrono::Utc::now().naive_utc()),
                quota.eq(original + 1),
            ))
            .execute(connection)
            .await
            .map(|_| ())
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                ProteinError::Database(error.to_string())
            })
    }

    pub async fn delete(
        key: Uuid,
        connection: &mut DatabaseConnection,
    ) -> Result<usize, ProteinError> {
        diesel::delete(api_keys::table)
            .filter(api_key.eq(key))
            .execute(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                ProteinError::Database(error.to_string())
            })
    }

    pub async fn find(
        key: Uuid,
        connection: &mut DatabaseConnection,
    ) -> Result<APIKey, ProteinError> {
        api_keys::table
            .filter(api_key.eq(key))
            .select(APIKey::as_select())
            .first(connection)
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

    pub async fn verify(
        user: Uuid,
        key: Uuid,
        mut connection: DatabaseConnection,
    ) -> Result<bool, ProteinError> {
        // First, Find User
        let user: User = User::find(user, &mut connection).await?;

        // Next, Grab API Key
        let verified: APIKey = APIKey::get(&user, &mut connection).await?;

        // Extra Validation: Quota Check
        if verified.quota >= API_QUOTA_LIMIT {
            return Err(ProteinError::Authorization(
                "API Key Quota Limit Reached!".to_string(),
            ));
        }

        // Extra Validation: Status Check
        if verified.status != Status::Active {
            return Err(ProteinError::Authorization(
                "API Key is Revoked OR Expired!".to_string(),
            ));
        }

        // Extra Validation: Expiry Check
        let now: i64 = chrono::Utc::now().naive_utc().and_utc().timestamp();
        let expired: i64 = verified.expires_at.and_utc().timestamp();

        if now > expired {
            return Err(ProteinError::Authorization(
                "API Key is Expired!".to_string(),
            ));
        }

        // Most importantly, Compare API_KEY Header to Actual API Key, As well as
        // checking for developer key/admin
        if (verified.api_key == key && verified.user_id == user.id) || verified.is_developer_key {
            APIKey::increment(key, verified.quota, &mut connection).await?;

            return Ok(true);
        } else {
            return Err(ProteinError::Authorization(
                "API Key Does Not Match!".to_string(),
            ));
        }
    }
}
