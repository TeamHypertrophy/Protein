// Rocket
use rocket::get;
use rocket::State;
use rocket::serde::json::Json;

// Diesel
use diesel::prelude::*;

// Protein
use crate::models::user::User;

use crate::db::DatabasePool;
use crate::schema::users::dsl::*;


#[get("/get/<user_id>", format = "json")]
pub async fn get(user_id: i32, pool: &State<DatabasePool>) -> Option<Json<User>> {

    // [?] Connecting to Database Pool
    let pool = pool;

    // [=] Creating Database Connection
    let conn = &mut pool.get().expect("Failed");

    // [>] Fetch User and Return
    users
        .find(user_id)
        .select(User::as_select())
        .first(conn)
        .map(Json)
        .ok()
}

