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

// Users
use crate::{
    schema::users, schema::users::dsl::*, db::DatabaseConnection, models::keys::APIKey,
    responders::ProteinError,
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
#[serde(crate = "rocket::serde")]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub is_dev: bool,
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
        connection: &mut DatabaseConnection,
    ) -> Result<User, ProteinError> {
        diesel::update(users::table)
            .filter(id.eq(user_id))
            .set((username.eq(new_username),))
            .get_result::<User>(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                ProteinError::Database(error.to_string())
            })
    }
}

// New User Model
#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUser {
    pub username: String,
    pub is_dev: bool,
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
pub struct CreatedUser {
    pub user: User,
    pub api_key: Uuid,
}

#[derive(Serialize, Deserialize)]
pub struct Me {
    pub user: User,
    pub api_key: APIKey,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateUser {
    pub username: String,
}
