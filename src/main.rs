// Rocket Macro
#[macro_use] extern crate rocket;

// Schema File
mod schema;

// Database
mod models;
mod db;

// Redis
mod cache;

// API Routes
pub mod api;

// Fairings
pub mod fairings;

// Error Handlers
pub mod errors;

// Shadow
use shadow_rs::shadow;

shadow!(build);

// Tracing
use std::path::Path;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::fmt::writer::MakeWriterExt;
use tracing_subscriber;

// Launch Rocket Instance
#[launch]
async fn protein() -> _ {
    let path = Path::new("./logs/");

    let appender = RollingFileAppender::builder()
        .rotation(Rotation::DAILY)
        .filename_suffix("[Protein].log")
        .build(path)
        .expect("[!] Error Building Log Files");

    let (non_blocking_appender, _guard) = tracing_appender::non_blocking(appender);
    let stdout = std::io::stdout.with_max_level(tracing::Level::INFO);

    tracing_subscriber::fmt()
        .with_writer(stdout.and(non_blocking_appender))
        .init();

    let pool = db::establish_connection().await.ok().unwrap();
    let redis = cache::redis::build_redis().await.ok().unwrap();


    rocket::build()
        .manage(pool)
        .manage(redis)
        .attach(fairings::cors::Cors)
        .attach(fairings::logging::Logging)
        .mount("/", routes![api::index::index])
        .mount("/health", routes![api::health::redis, api::health::postgres])
        .mount("/system", routes![api::system::rust, api::system::package, api::system::git])
        .mount("/v1/users", routes![api::users::get, api::users::all])
        .register("/", catchers![errors::default, errors::not_found])
}
