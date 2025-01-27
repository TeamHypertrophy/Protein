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

use crate::auth::rate_limit::RateLimit;

#[get("/get/<user_id>/<workout_id>")]
pub async fn get(_r: RateLimit<'_>, user_id: Uuid, workout_id: Uuid) -> String {
    format!("GET: {}", workout_id)
}

#[get("/all/<user_id>")]
pub async fn all(_r: RateLimit<'_>, user_id: Uuid) -> &'static str {
    "ALL"
}

#[post("/update/<user_id>/<workout_id>")]
pub async fn update(_r: RateLimit<'_>, user_id: Uuid, workout_id: Uuid) -> String {
    format!("UPDATE: {}", workout_id)
}

#[post("/create")]
pub async fn create(_r: RateLimit<'_>) -> &'static str {
    "CREATE"
}

#[post("/delete/<user_id>/<workout_id>")]
pub async fn delete(_r: RateLimit<'_>, user_id: Uuid, workout_id: Uuid) -> String {
    format!("DELETE: {}", workout_id)
}
