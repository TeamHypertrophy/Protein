// Rocket
use rocket::get;
use rocket::State;
use rocket::serde::json::Json;

// Diesel
use diesel::prelude::*;

// Diesel Async
use diesel_async::RunQueryDsl;

// Protein
use crate::models::user::User;
use crate::db::DatabasePool;
use crate::schema::users::dsl::*;


#[get("/get/<user_id>", format = "json")]
pub async fn get(user_id: i32, pool: &State<DatabasePool>) -> Option<Json<User>> {
    // [=] Creating Database Connection
    let conn = &mut pool.get().await.ok()?;

    // [>] Fetch User and Return
    users
        .find(user_id)
        .select(User::as_select())
        .first(conn)
        .await
        .ok()
        .map(Json)
}

#[get("/all", format = "json")]
pub async fn all(pool: &State<DatabasePool>) -> Option<Json<Vec<User>>> {
    // [=] Creating Database Connection
    let conn = &mut pool.get().await.ok()?;

    // [>] Fetch List of Users and Return
    users
        .select(User::as_select())
        .load(conn)
        .await
        .ok()
        .map(Json)
}
