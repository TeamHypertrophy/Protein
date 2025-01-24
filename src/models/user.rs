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
    models::keys::APIKey,
    responders::ProteinError,
    schema::{users, users::dsl::*},
};

// User Model
#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    Queryable,
    Selectable,
    Serialize,
    Deserialize,
    AsChangeset,
    Identifiable,
)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password: String,
    pub password_updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub role: Role,
    pub ip_address: String,
}

impl User {
    pub async fn find(
        user_id: Uuid,
        connection: &mut DatabaseConnection,
    ) -> Result<User, ProteinError> {
        users::table
            .find(user_id)
            .select(User::as_select())
            .first(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                ProteinError::Database(error.to_string())
            })
    }

    pub async fn find_by_username(
        name: String,
        connection: &mut DatabaseConnection,
    ) -> Result<User, ProteinError> {
        users::table
            .filter(username.eq(name))
            .select(User::as_select())
            .first(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                ProteinError::Database(error.to_string())
            })
    }

    pub async fn find_by_ip(
        address: String,
        connection: &mut DatabaseConnection,
    ) -> Result<User, ProteinError> {
        users::table
            .filter(ip_address.eq(address))
            .select(User::as_select())
            .first(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                ProteinError::Database(error.to_string())
            })
    }

    pub async fn all(connection: &mut DatabaseConnection) -> Result<Vec<User>, ProteinError> {
        users::table
            .select(User::as_select())
            .load(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                ProteinError::Database(error.to_string())
            })
    }

    pub async fn delete(
        user_id: Uuid,
        connection: &mut DatabaseConnection,
    ) -> Result<usize, ProteinError> {
        diesel::delete(users::table.filter(id.eq(user_id)))
            .execute(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                ProteinError::Database(error.to_string())
            })
    }

    pub async fn update(
        user_id: Uuid,
        new_username: String,
        new_password: String,
        connection: &mut DatabaseConnection,
    ) -> Result<User, ProteinError> {
        diesel::update(users::table)
            .filter(id.eq(user_id))
            .set((username.eq(new_username), password.eq(new_password)))
            .get_result::<User>(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                ProteinError::Database(error.to_string())
            })
    }
}

#[derive(DbEnum, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[ExistingTypePath = "crate::schema::sql_types::Role"]
pub enum Role {
    Developer,
    Admin,
    Trainer,
    User,
}

// New User Model
#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub role: Role,
    pub ip_address: String,
}

impl NewUser {
    pub async fn create(
        connection: &mut DatabaseConnection,
        user: NewUser,
    ) -> Result<User, ProteinError> {
        diesel::insert_into(users::table)
            .values(&user)
            .get_result::<User>(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                ProteinError::Database(error.to_string())
            })
    }
}

#[derive(Serialize, Deserialize)]
pub struct ProteinUser {
    pub user: User,
    pub api_key: Uuid,
}

#[derive(Serialize, Deserialize)]
pub struct LoginUser {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct Me {
    pub user: User,
    pub api_key: APIKey,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateUser {
    pub username: String,
    pub password: String,
}
