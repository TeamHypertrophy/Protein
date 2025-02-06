/*
______          _       _
| ___ \        | |     (_)
| |_/ / __ ___ | |_ ___ _ _ __
|  __/ '__/ _ \| __/ _ \ | '_ \
| |  | | | (_) | ||  __/ | | | |
\_|  |_|  \___/ \__\___|_|_| |_|

        Made with ❤️
*/

use uuid::Uuid;
use rocket::{get, post};

use crate::auth::rate_limit::RateLimit;

// Exercises

#[get("/exercise/get/<exercise_id>?<user_id>")]
pub async fn exercise_get(_r: RateLimit<'_>, exercise_id: i64, user_id: Uuid) -> &'static str {
    "GET"
}

#[get("/exercise/all")]
pub async fn exercise_all(_r: RateLimit<'_>) -> &'static str {
    "ALL"
}

#[get("/exercise/all?<user_id>")]
pub async fn exercise_user_all(_r: RateLimit<'_>, user_id: Uuid) -> &'static str {
    "ALL"
}

#[post("/exercise/update/<exercise_id>?<user_id>")]
pub async fn exercise_update(_r: RateLimit<'_>, exercise_id: i64, user_id: Uuid) -> &'static str {
    "UPDATE"
}

#[post("/exercise/create?<user_id>")]
pub async fn exercise_create(_r: RateLimit<'_>, user_id: Uuid) -> &'static str {
    "CREATE"
}

#[post("/exercise/delete/<exercise_id>?<user_id>")]
pub async fn exercise_delete(_r: RateLimit<'_>, exercise_id: i64, user_id: Uuid) -> &'static str {
    "DELETE"
}

// Workouts

#[get("/workout/get/<workout_id>?<user_id>")]
pub async fn workout_get(_r: RateLimit<'_>, workout_id: i64, user_id: Uuid) -> &'static str {
    "GET"
}

#[get("/workout/all")]
pub async fn workout_all(_r: RateLimit<'_>) -> &'static str {
    "ALL"
}

#[get("/workout/all?<user_id>")]
pub async fn workout_user_all(_r: RateLimit<'_>, user_id: Uuid) -> &'static str {
    "ALL"
}

#[post("/workout/update/<workout_id>?<user_id>")]
pub async fn workout_update(_r: RateLimit<'_>, workout_id: i64, user_id: Uuid) -> &'static str {
    "UPDATE"
}

#[post("/workout/create?<user_id>")]
pub async fn workout_create(_r: RateLimit<'_>, user_id: Uuid) -> &'static str {
    "CREATE"
}

#[post("/workout/delete/<workout_id>?<user_id>")]
pub async fn workout_delete(_r: RateLimit<'_>, workout_id: i64, user_id: Uuid) -> &'static str {
    "DELETE"
}
