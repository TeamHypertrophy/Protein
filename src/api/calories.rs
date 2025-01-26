/*
______          _       _
| ___ \        | |     (_)
| |_/ / __ ___ | |_ ___ _ _ __
|  __/ '__/ _ \| __/ _ \ | '_ \
| |  | | | (_) | ||  __/ | | | |
\_|  |_|  \___/ \__\___|_|_| |_|

        Made with ❤️
*/

use rocket::{get, post};
use uuid::Uuid;

#[get("/get/<user_id>/<log_id>")]
pub async fn get(user_id: Uuid, log_id: i32) -> &'static str {
    "GET"
}

#[get("/all")]
pub async fn all() -> &'static str {
    "ALL"
}

#[post("/update/<user_id>/<log_id>")]
pub async fn update(user_id: Uuid, log_id: i32) -> &'static str {
    "UPDATE"
}

#[post("/create")]
pub async fn create() -> &'static str {
    "CREATE"
}

#[post("/delete/<user_id>/<log_id>")]
pub async fn delete(user_id: Uuid, log_id: i32) -> &'static str {
    "DELETE"
}
