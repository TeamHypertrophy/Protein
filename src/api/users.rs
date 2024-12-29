/*
______          _       _       
| ___ \        | |     (_)      
| |_/ / __ ___ | |_ ___ _ _ __  
|  __/ '__/ _ \| __/ _ \ | '_ \ 
| |  | | | (_) | ||  __/ | | | |
\_|  |_|  \___/ \__\___|_|_| |_|

        Made with ❤️ 
*/

// Rocket
use rocket::serde::json::{Json, Value};
use rocket::serde::uuid::Uuid;
use rocket::http::Status;
use rocket::{get, post, State};

// Protein
use crate::{
    cache::redis::{RedisPool, Cache}, models::user::{NewUser, User}, db::DatabasePool,
};

#[get("/<user_id>", format = "application/json")]
pub async fn get(user_id: Uuid, pool: &State<DatabasePool>, redis: &State<RedisPool>) -> Result<Json<User>, Status> {
    let cache: Value = Cache::get(redis, user_id.to_string())
        .await
        .expect("tuff");

    if cache.is_null() {
        let connection = &mut pool.get()
            .await
            .map_err(|_| Status::InternalServerError)
            .unwrap();

        let user = User::find(user_id, connection)
            .await
            .map_err(|_| Status::InternalServerError)
            .unwrap();

        Cache::set(redis, user_id.to_string(), Cache::serialize(&user))
            .await
            .expect("[!] Failed to Set Cache");

        Ok(Json(user))
    } else {
        let user: User = Cache::deserialize(cache);

        Ok(Json(user))
    }
}

#[get("/all", format = "application/json")]
pub async fn all(pool: &State<DatabasePool>) -> Option<Json<Vec<User>>> {
    // [=] Creating Database Connection
    let connection = &mut pool.get().await.ok()?;

    // [>] Fetch List of Users and Return
    User::all(connection)
        .await
        .ok()
        .map(Json)
}

#[post("/create", format = "application/json", data = "<user>")]
pub async fn create(pool: &State<DatabasePool>, redis: &State<RedisPool>, user: Json<NewUser>) -> Json<User> {
    todo!()
}

#[post("/delete/<user_id>", format = "application/json")]
pub async fn delete(user_id: Uuid, pool: &State<DatabasePool>) -> Json<User> {
    todo!()
}

#[post("/update/<user_id>", format = "application/json", data = "<user>")]
pub async fn update(user_id: Uuid, pool: &State<DatabasePool>, redis: &State<RedisPool>, user: Json<User>) -> Json<User> {
    todo!()
}