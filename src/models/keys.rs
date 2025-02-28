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
    db::DBConnection,
    errors::Error,
    models::user::{Role, User},
    schema::{
        api_keys,
        api_keys::dsl::{api_key, quota, revoked_reason, role, status, user_id},
    },
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
#[diesel(primary_key(key_id))]
#[diesel(table_name = api_keys)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(User))]
pub struct APIKey {
    pub key_id: i32,
    pub user_id: Uuid,
    pub api_key: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub expires_at: NaiveDateTime,
    pub role: Role,
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
    pub expires_at: Option<NaiveDateTime>,
    pub role: Option<Role>,
    pub revoked_reason: Option<String>,
    pub status: Option<Status>,
    pub quota: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RevokeKey {
    pub revoked_reason: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateRole {
    pub role: Role,
}

impl APIKey {
    pub async fn get(user: &User, connection: &mut DBConnection) -> Result<APIKey, Error> {
        APIKey::belonging_to(user)
            .select(APIKey::as_select())
            .first(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn all(connection: &mut DBConnection) -> Result<Vec<APIKey>, Error> {
        api_keys::table
            .select(APIKey::as_select())
            .load(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn user_all(user: Uuid, connection: &mut DBConnection) -> Result<Vec<APIKey>, Error> {
        api_keys::table
            .filter(user_id.eq(user))
            .select(APIKey::as_select())
            .load(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn revoke(
        key: Uuid,
        data: RevokeKey,
        connection: &mut DBConnection,
    ) -> Result<APIKey, Error> {
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
                Error::Database(error.to_string())
            })
    }

    pub async fn generate(user: &User, connection: &mut DBConnection) -> Result<APIKey, Error> {
        diesel::insert_into(api_keys::table)
            .values(user_id.eq(user.user_id))
            .get_result(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn update(
        key: Uuid,
        data: UpdateAPIKey,
        connection: &mut DBConnection,
    ) -> Result<APIKey, Error> {
        diesel::update(api_keys::table)
            .filter(api_key.eq(key))
            .set(&data)
            .get_result::<APIKey>(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn change_role(
        key: Uuid,
        data: UpdateRole,
        connection: &mut DBConnection,
    ) -> Result<APIKey, Error> {
        diesel::update(api_keys::table)
            .filter(api_key.eq(key))
            .set(role.eq(data.role))
            .get_result::<APIKey>(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn increment(
        key: Uuid,
        original: i32,
        connection: &mut DBConnection,
    ) -> Result<(), Error> {
        diesel::update(api_keys::table)
            .filter(api_key.eq(key))
            .set(quota.eq(original + 1))
            .execute(connection)
            .await
            .map(|_| ())
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn delete(key: Uuid, connection: &mut DBConnection) -> Result<usize, Error> {
        diesel::delete(api_keys::table)
            .filter(api_key.eq(key))
            .execute(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn find(key: Uuid, connection: &mut DBConnection) -> Result<APIKey, Error> {
        api_keys::table
            .filter(api_key.eq(key))
            .select(APIKey::as_select())
            .first(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn find_by_key(key: Uuid, mut connection: DBConnection) -> Result<APIKey, Error> {
        api_keys::table
            .filter(api_key.eq(key))
            .select(APIKey::as_select())
            .first(&mut connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn verify(
        user: Uuid,
        key: Uuid,
        mut connection: DBConnection,
    ) -> Result<bool, Error> {
        // First, Find User
        let user: User = User::find(user, &mut connection).await?;

        // Next, Grab API Key
        let verified: APIKey = APIKey::get(&user, &mut connection).await?;

        // Extra Validation: Quota Check
        if verified.quota >= API_QUOTA_LIMIT {
            return Err(Error::Authorization(
                "API Key Quota Limit Reached!".to_string(),
            ));
        }

        // Extra Validation: Status Check
        if verified.status != Status::Active {
            return Err(Error::Authorization(
                "API Key is Revoked OR Expired!".to_string(),
            ));
        }

        // Extra Validation: Expiry Check
        let now: i64 = chrono::Utc::now().naive_utc().and_utc().timestamp();
        let expired: i64 = verified.expires_at.and_utc().timestamp();

        if now > expired {
            return Err(Error::Authorization("API Key is Expired!".to_string()));
        }

        // Most importantly, Compare API_KEY Header to Actual API Key, As well as
        // checking for developer key/admin
        if (verified.api_key == key && verified.user_id == user.user_id)
            || verified.role == Role::Admin
            || verified.role == Role::Developer
        {
            APIKey::increment(key, verified.quota, &mut connection).await?;

            Ok(true)
        } else {
            Err(Error::Authorization("API Key Does Not Match!".to_string()))
        }
    }
}
