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
use rocket::get;

// Index Response
#[get("/")]
pub async fn index() -> &'static str {
    "[!] Welcome to Protein, Hypertrophy's Rust API Backend :)"
}
