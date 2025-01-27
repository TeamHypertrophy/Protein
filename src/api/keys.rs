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

#[get("/get/<api_key>")]
pub async fn get(_r: RateLimit<'_>, api_key: Uuid) -> &'static str {
    "GET"
}

#[get("/all")]
pub async fn all(_r: RateLimit<'_>) -> &'static str {
    "ALL"
}

#[post("/update/<api_key>")]
pub async fn update(_r: RateLimit<'_>, api_key: Uuid) -> &'static str {
    "UPDATE"
}

#[post("/revoke/<api_key>")]
pub async fn revoke(_r: RateLimit<'_>, api_key: Uuid) -> &'static str {
    "REVOKE"
}

#[post("/delete/<api_key>")]
pub async fn delete(_r: RateLimit<'_>, api_key: Uuid) -> &'static str {
    "DELETE"
}
