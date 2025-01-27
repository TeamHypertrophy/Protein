/*
______          _       _
| ___ \        | |     (_)
| |_/ / __ ___ | |_ ___ _ _ __
|  __/ '__/ _ \| __/ _ \ | '_ \
| |  | | | (_) | ||  __/ | | | |
\_|  |_|  \___/ \__\___|_|_| |_|

        Made with ❤️
*/

use rocket::get;

use crate::auth::rate_limit::RateLimit;

#[get("/")]
pub async fn index(_r: RateLimit<'_>) -> &'static str {
    "[!] Welcome to Protein, Hypertrophy's Rust API Backend :)"
}
