// Rocket
use rocket::get;

// API Check
#[get("/")]
pub async fn api() -> &'static str {
    "[!] TODO"
}

#[get("/redis")]
pub async fn redis() -> &'static str {
    "[!] TODO"
}

#[get("/postgres")]
pub async fn postgres() -> &'static str {
    "[!] TODO"
}
