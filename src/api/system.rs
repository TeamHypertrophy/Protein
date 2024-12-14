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
        "CARGO_VERSION": build::CARGO_VERSION,
    })
}

#[get("/package", format = "json")]
pub async fn package() -> Value {
    json!({
        "BUILD_OS": build::BUILD_OS,
        "PROJECT_NAME": build::PROJECT_NAME,
        "BUILD_TIME": build::BUILD_TIME,
        "BUILD_RUST_CHANNEL": build::BUILD_RUST_CHANNEL,
        "BRANCH": build::BRANCH,
        "PKG_VERSION": build::PKG_VERSION,
    })
}

#[get("/git", format = "json")]
pub async fn git() -> Value {
    json!({
        "GIT_BRANCH": shadow_rs::branch(),
        "GIT_TAG": shadow_rs::tag(),
        "GIT_CLEAN": shadow_rs::git_clean(),
        "GIT_COMMIT": build::SHORT_COMMIT,
        "GIT_COMMIT_DATE": build::COMMIT_DATE_2822,
    })
}
