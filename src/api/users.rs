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
    println!("[-] GET Request for api/v1/users/get/user_id/{}", user_id);

    // Check if Data is in Cache
    println!("[?] Checking If User Is In Redis Cache");

    let cache: Value = redis.get(user_id.to_string()).await.expect("[-] Getting User In Cache Failed!");

    // User Variable Data
    let data: Option<User>;

    // Check if Cache is Empty
    if cache.is_null() {
        println!("[?] Redis Cache is Null");

        // [=] Creating Database Connection
        println!("[?] Creating Database Connection");
        let conn = &mut pool.get().await.ok()?;
        println!("[!] Database Connection Created");

        // [>] Fetch User and Return
        println!("[?] Fetching User From Database");
        data = users
            .find(user_id)
            .select(User::as_select())
            .first(conn)
            .await
            .ok();
        println!("[!] Got User From Database");

        // [=] Set User Data in Cache
        println!("[-] Setting User Data to Redis Cache!");
        let serialized_user: String = json::to_string(&data).unwrap();
        let _: () = redis.set(user_id.to_string(), serialized_user, None, None, false)
            .await
            .expect("[!] Failed to Set User Data In Cache!");
        println!("[!] User Data Set In Redis Cache");

    } else {
        println!("[!!] Cache is Not Empty: {:?}", cache);

        println!("[!] Returning User JSON");
        let cache_user: User = json::from_value(cache).unwrap();
        return Some(cache_user).map(Json);
    }
    data.map(Json)
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
