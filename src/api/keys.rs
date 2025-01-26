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

#[get("/get/<api_key>")]
pub async fn get(api_key: Uuid) -> &'static str {
    "GET"
}

#[get("/all")]
pub async fn all() -> &'static str {
    "ALL"
}

#[post("/update/<api_key>")]
pub async fn update(api_key: Uuid) -> &'static str {
    "UPDATE"
}

#[post("/revoke/<api_key>")]
pub async fn revoke(api_key: Uuid) -> &'static str {
    "REVOKE"
}

#[post("/delete/<api_key>")]
pub async fn delete(api_key: Uuid) -> &'static str {
    "DELETE"
}
