/*
______          _       _       
| ___ \        | |     (_)      
| |_/ / __ ___ | |_ ___ _ _ __  
|  __/ '__/ _ \| __/ _ \ | '_ \ 
| |  | | | (_) | ||  __/ | | | |
\_|  |_|  \___/ \__\___|_|_| |_|

        Made with ❤️ 
*/

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
    pub id: i32,
    pub username: String,
    pub is_dev: bool,
}

// New User Model
#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser<'a> {
    pub id: i32,
    pub username: &'a str,
}
