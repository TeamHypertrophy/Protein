// Diesel
use diesel::prelude::*;

// Serde Serialization
use rocket::serde::{Serialize, Deserialize};

// User Model
#[derive(Queryable, Selectable)]
#[derive(Serialize, Deserialize)]
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
    pub id: &'a i32,
    pub username: &'a str,
}