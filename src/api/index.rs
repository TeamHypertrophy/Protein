// Rocket
use rocket::get;

// Index Response
#[get("/")]
pub async fn index() -> &'static str {
    "[!] Welcome to Protein, HyperTrophy's Rust API Backend :)"
}