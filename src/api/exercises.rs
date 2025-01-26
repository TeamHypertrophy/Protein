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

#[get("/get/<exercise_id>")]
pub async fn get(exercise_id: i64) -> &'static str {
    "GET"
}

#[get("/all")]
pub async fn all() -> &'static str {
    "ALL"
}

#[post("/update/<exercise_id>")]
pub async fn update(exercise_id: i64) -> &'static str {
    "UPDATE"
}

#[post("/create")]
pub async fn create() -> &'static str {
    "CREATE"
}

#[post("/delete/<exercise_id>")]
pub async fn delete(exercise_id: i64) -> &'static str {
    "DELETE"
}
