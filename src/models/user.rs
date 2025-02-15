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
    responders::ProteinError,
    schema::{users, users::dsl::{user_id, username, password, password_updated_at, created_at, role, ip_address}},
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
#[diesel(primary_key(user_id))]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub user_id: Uuid,
    pub username: String,
    pub password: String,
    pub password_updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub role: Role,
    pub ip_address: String,
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

#[derive(AsChangeset)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Serialize, Deserialize)]
pub struct UpdateUser {
    pub username: Option<String>,
    pub role: Option<Role>,
    pub ip_address: Option<String>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub ip_address: String,
}

#[derive(Serialize, Deserialize)]
pub struct Password {
    pub old_password: String,
    pub new_password: String,
}

#[derive(DbEnum, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[ExistingTypePath = "crate::schema::sql_types::Role"]
pub enum Role {
    Developer,
    Admin,
    Trainer,
    User,
}

impl User {
    pub async fn find(id: Uuid, connection: &mut DatabaseConnection) -> Result<User, ProteinError> {
        users::table
            .find(id)
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
        id: Uuid,
        connection: &mut DatabaseConnection,
    ) -> Result<usize, ProteinError> {
        diesel::delete(users::table.filter(user_id.eq(id)))
            .execute(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                ProteinError::Database(error.to_string())
            })
    }

    pub async fn update(
        id: Uuid,
        data: UpdateUser,
        connection: &mut DatabaseConnection,
    ) -> Result<User, ProteinError> {
        diesel::update(users::table)
            .filter(user_id.eq(id))
            .set(&data)
            .get_result::<User>(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                ProteinError::Database(error.to_string())
            })
    }

    pub async fn create(
        connection: &mut DatabaseConnection,
        data: NewUser,
    ) -> Result<User, ProteinError> {
        diesel::insert_into(users::table)
            .values(&data)
            .get_result::<User>(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                ProteinError::Database(error.to_string())
            })
    }

    pub async fn update_password(
        id: Uuid,
        new_password: &String,
        connection: &mut DatabaseConnection,
    ) -> Result<User, ProteinError> {
        diesel::update(users::table)
            .filter(user_id.eq(id))
            .set((
                password.eq(new_password),
                password_updated_at.eq(chrono::Utc::now().naive_utc()),
            ))
            .get_result::<User>(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                ProteinError::Database(error.to_string())
            })
    }
}
