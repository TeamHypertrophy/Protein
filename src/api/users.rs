// Rocket
use rocket::get;
use rocket::State;
use rocket::serde::json;
use rocket::serde::json::{Json, Value};

// Diesel
use diesel::prelude::*;

// Diesel Async
use diesel_async::RunQueryDsl;

// Protein
use crate::models::user::User;
use crate::db::DatabasePool;
use crate::cache::redis::RedisPool;
use crate::schema::users::dsl::*;

// Redis
use fred::prelude::*;

#[get("/get/<user_id>", format = "json")]
pub async fn get(user_id: i32, pool: &State<DatabasePool>, redis: &State<RedisPool>) -> Option<Json<User>> {
    // [?] Check if User Data is in Redis Cache
    let cache: Value = redis.get(user_id.to_string()).await.expect("[-] Getting User In Cache Failed!");

    // [-] User Variable
    let data: Option<User>;

    // [?] Check if Redis Cache is Empty
    if cache.is_null() {

        // [=] Create Database Connection
        let connection = &mut pool.get().await.ok()?;

        // [>] Fetch User from Database
        data = users
            .find(user_id)
            .select(User::as_select())
            .first(connection)
            .await
            .ok();

        // [-] Serialize User Object
        let serialized_user: String = json::to_string(&data).unwrap();

        // [=] Set User Data in Cache
        let _: () = redis.set(user_id.to_string(), serialized_user, None, None, false)
            .await
            .expect("[!] Failed to Set User Data In Cache!");

        // [>] Return User
        data.map(Json)
    } else {
        // [-] Deserialize User
        let user: User = json::from_value(cache).unwrap();

        // [>] Return User
        return Some(user).map(Json);
    }
}

#[get("/all", format = "json")]
pub async fn all(pool: &State<DatabasePool>) -> Option<Json<Vec<User>>> {
    // [=] Creating Database Connection
    let connection = &mut pool.get().await.ok()?;

    // [>] Fetch List of Users and Return
    users
        .select(User::as_select())
        .load(connection)
        .await
        .ok()
        .map(Json)
}
