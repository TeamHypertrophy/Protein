// Rocket
use rocket::get;

// Index Response
#[get("/")]
pub async fn index() -> &'static str {
    "[!] Welcome to Protein, Hypertrophy's Rust API Backend :)"
}
