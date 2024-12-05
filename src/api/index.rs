// Rocket
use rocket::get;

// Reqwest
use reqwest;

// Index Response
#[get("/")]
pub async fn index() -> &'static str {
    "[!] Welcome to Protein, HyperTrophy's Rust API Backend :)"
}

// Get 102 Loading Cat Picture (Yes This is Ugly.)
#[get("/cat")]
pub async fn cat() -> String {
    let body = reqwest::get("https://http.cat/102")
        .await
        .ok()
        .unwrap()
        .text()
        .await
        .ok()
        .unwrap();
    body
}
