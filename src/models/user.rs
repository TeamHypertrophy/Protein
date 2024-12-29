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
use crate::{schema::users, db::DatabaseConnection};

// User Model
#[derive(Clone, Debug, Eq, PartialEq, Queryable, Selectable, Serialize, Deserialize, AsChangeset)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub is_dev: bool,
}

impl User {
    pub async fn find(user_id: Uuid, connection: &mut DatabaseConnection) -> Result<User, diesel::result::Error> {
        users::table
            .find(user_id)
            .select(User::as_select())
            .first(connection)
            .await
    }

    pub async fn all(connection: &mut DatabaseConnection) -> Result<Vec<User>, diesel::result::Error> {
        users::table
            .select(User::as_select())
            .load(connection)
            .await
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
    pub async fn create(&self, connection: &mut DatabaseConnection) -> Result<User, diesel::result::Error> {
        todo!()
    }
}