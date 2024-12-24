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

// Serde Serialization
use rocket::serde::{Deserialize, Serialize};

// User Model
#[derive(Clone, Debug, Eq, PartialEq, Queryable, Selectable, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub is_dev: bool,
}

// New User Model
#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser<'a> {
    pub id: Uuid,
    pub username: &'a str,
}
