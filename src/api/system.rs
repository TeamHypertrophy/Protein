// Rocket
use rocket::get;
use rocket::serde::json::{json, Value};

// Shadow
use crate::build;

#[get("/rust", format = "json")]
pub async fn rust() -> Value {
    json!({
        "RUST_VERSION": build::RUST_VERSION,
        "RUST_CHANNEL": build::RUST_CHANNEL,
    })
}
