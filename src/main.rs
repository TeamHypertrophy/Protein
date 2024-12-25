/*
______          _       _       
| ___ \        | |     (_)      
| |_/ / __ ___ | |_ ___ _ _ __  
|  __/ '__/ _ \| __/ _ \ | '_ \ 
| |  | | | (_) | ||  __/ | | | |
\_|  |_|  \___/ \__\___|_|_| |_|

        Made with ❤️ 
*/

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

// Constants
pub mod constants;

// Shadow
use shadow_rs::shadow;

shadow!(build);

// Tracing
use std::path::Path;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{fmt, prelude::*};
use tracing_subscriber::fmt::format::FmtSpan;

use rocket::fairing::AdHoc;

// Launch Rocket Instance
#[launch]
async fn protein() -> _ {
    // Build Path for Logs Directory
    let path = Path::new("./logs/");

    // This Creates a Log File that Rotates Daily
    let appender = RollingFileAppender::builder()
        .rotation(Rotation::DAILY)
        .filename_suffix("[Protein].log")
        .build(path)
        .expect("[!] Error Building Log Files");

    let (non_blocking_appender, _guard) = tracing_appender::non_blocking(appender);

    // Seperate Layers for File and Terminal Logging
    let file_layer = fmt::layer()
        .with_writer(non_blocking_appender)
        .with_ansi(false)
        .with_span_events(FmtSpan::CLOSE);

    let terminal_layer = fmt::layer()
        .with_ansi(true)
        .without_time()
        .with_span_events(FmtSpan::CLOSE);

    // Register Layers
    tracing_subscriber::registry()
        .with(file_layer)
        .with(terminal_layer)
        .init();

    let pool = match db::establish_connection().await {
        Ok(pool) => pool,
        Err(e) => {
            tracing::error!("[-] Error Connecting to Database: {:?}", e);
            panic!("[!] Error Connecting to Database - Aborting");
        }
    };

    let redis = match cache::redis::build_redis().await {
        Ok(redis) => redis,
        Err(e) => {
            tracing::error!("[-] Error Connecting to Redis: {:?}", e);
            panic!("[!] Error Connecting to Redis - Aborting");
        }
    };


    rocket::build()
        .manage(pool)
        .manage(redis)
        .attach(fairings::cors::Cors)
        .attach(fairings::logging::Logging)
        .attach(AdHoc::on_shutdown("[!] Write Logs", |_| Box::pin(async move {
            drop(_guard)
        })))
        .mount("/", routes![api::index::index])
        .mount(
            "/health", 
            routes![api::health::redis, api::health::postgres]
        )
        .mount(
            "/system", 
            routes![api::system::rust, api::system::package, api::system::git]
        )
        .mount(
            "/v1/users", 
            routes![api::users::get, api::users::all]
        )
        .register(
            "/", 
            catchers![errors::default, errors::not_found]
        )
}
