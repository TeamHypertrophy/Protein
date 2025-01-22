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

#[get("/<user_id>", format = "application/json")]
pub async fn get(user_id: String) -> String {
    format!("User Profile: {}", user_id)
}

#[post("/create", format = "application/json")]
pub async fn create() -> String {
    format!("Create Profile ")
}

#[post("/update/<user_id>", format = "application/json")]
pub async fn update(user_id: String) -> String {
    format!("Update Profile: {}", user_id)
}

#[get("/delete/<user_id>", format = "application/json")]
pub async fn delete(user_id: String) -> String {
    format!("Delete Profile: {}", user_id)
}
