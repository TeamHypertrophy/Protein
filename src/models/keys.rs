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
    models::{
        trainer::Trainer,
        user::{Role, User},
    },
    schema::{api_key_logs, api_keys},
    utils::admin::{self, RequestInfo},
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

    pub async fn get_current(user: &User, connection: &mut DBConnection) -> Result<APIKey, Error> {
        APIKey::belonging_to(user)
            .select(APIKey::as_select())
            .filter(api_keys::status.eq(Status::Active))
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
            .filter(api_keys::user_id.eq(user))
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
            .filter(api_keys::api_key.eq(key))
            .set((
                api_keys::revoked_reason.eq(data.revoked_reason),
                api_keys::status.eq(Status::Revoked),
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
            .values(api_keys::user_id.eq(user.user_id))
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
            .filter(api_keys::api_key.eq(key))
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
            .filter(api_keys::api_key.eq(key))
            .set(api_keys::role.eq(data.role))
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
            .filter(api_keys::api_key.eq(key))
            .set(api_keys::quota.eq(original + 1))
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
            .filter(api_keys::api_key.eq(key))
            .execute(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn find(key: Uuid, connection: &mut DBConnection) -> Result<APIKey, Error> {
        api_keys::table
            .filter(api_keys::api_key.eq(key))
            .select(APIKey::as_select())
            .first(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn find_by_key(key: Uuid, connection: &mut DBConnection) -> Result<APIKey, Error> {
        api_keys::table
            .filter(api_keys::api_key.eq(key))
            .select(APIKey::as_select())
            .first(connection)
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
        request: RequestInfo,
    ) -> Result<bool, Error> {
        // First, Find User
        let user: User = User::find(user, &mut connection).await?;

        // Next, Grab API Key
        let verified: APIKey = APIKey::find(key, &mut connection).await?;

        let user_id = user.user_id.clone();
        let req = request.clone();

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
        // Check For Developer/Admin Key
        if (verified.api_key == key && verified.user_id == user.user_id)
            || verified.role == Role::Admin
            || verified.role == Role::Developer
        {
            APIKey::increment(key, verified.quota, &mut connection).await?;

            rocket::tokio::task::spawn(async move {
                admin::generate_api_key_log(req, connection, key, user_id, 200).await
            });

            Ok(true)
        } else {
            Err(Error::Authorization("API Key Does Not Match!".to_string()))
        }
    }

    pub async fn verify_trainer(
        id: i32,
        key: Uuid,
        mut connection: DBConnection,
        request: RequestInfo,
    ) -> Result<bool, Error> {
        // First, Find Trainer
        let trainer = Trainer::find(id, &mut connection).await?;

        // Find User Relating To Trainer
        let user: User = User::find(trainer.user_id, &mut connection).await?;

        // Next, Grab API Key
        let verified: APIKey = APIKey::find(key, &mut connection).await?;

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

            let req = request.clone();
            let user_id = user.user_id.clone();

            rocket::tokio::task::spawn(async move {
                admin::generate_api_key_log(req, connection, key, user_id, 200).await
            });

            Ok(true)
        } else {
            Err(Error::Authorization("API Key Does Not Match!".to_string()))
        }
    }
}

// APIKeyLog Model
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
#[diesel(primary_key(log_id))]
#[diesel(table_name = api_key_logs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(User))]
pub struct APIKeyLog {
    pub log_id: i32,
    pub user_id: Uuid,
    pub api_key: Uuid,
    pub method: String,
    pub route: String,
    pub status_code: i32,
    pub ip_address: String,
    pub user_agent: String,
    pub created_at: NaiveDateTime,
}

#[derive(AsChangeset, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = api_key_logs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UpdateAPIKeyLog {
    pub method: String,
    pub route: String,
    pub ip_address: String,
    pub user_agent: String,
    pub status_code: i32,
}

#[derive(AsChangeset, Insertable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = api_key_logs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewAPIKeyLog {
    pub user_id: Uuid,
    pub api_key: Uuid,
    pub method: String,
    pub route: String,
    pub status_code: i32,
    pub ip_address: String,
    pub user_agent: String,
}

impl APIKeyLog {
    pub async fn create(
        data: NewAPIKeyLog,
        connection: &mut DBConnection,
    ) -> Result<APIKeyLog, Error> {
        diesel::insert_into(api_key_logs::table)
            .values(&data)
            .get_result(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn all(connection: &mut DBConnection) -> Result<Vec<APIKeyLog>, Error> {
        api_key_logs::table
            .select(APIKeyLog::as_select())
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
    ) -> Result<Vec<APIKeyLog>, Error> {
        api_key_logs::table
            .filter(api_key_logs::user_id.eq(user))
            .select(APIKeyLog::as_select())
            .load(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn delete(key: i32, connection: &mut DBConnection) -> Result<usize, Error> {
        diesel::delete(api_key_logs::table)
            .filter(api_key_logs::log_id.eq(key))
            .execute(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn update(
        key: i32,
        data: UpdateAPIKeyLog,
        connection: &mut DBConnection,
    ) -> Result<APIKeyLog, Error> {
        diesel::update(api_key_logs::table)
            .filter(api_key_logs::log_id.eq(key))
            .set(&data)
            .get_result::<APIKeyLog>(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                Error::Database(error.to_string())
            })
    }
}
