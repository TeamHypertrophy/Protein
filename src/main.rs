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
#[macro_use]
extern crate rocket;

use rocket::fairing::AdHoc;

// Schema File
mod schema;

// Database
mod db;
mod models;

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

// Utils
pub mod utils;

// Responders
pub mod auth;
pub mod responders;

// Shadow
use shadow_rs::shadow;

shadow!(build);

// Launch Rocket Instance
#[launch]
async fn protein() -> _ {
    // Setup Logging, PostgreSQL and Redis
    let (guard, ()) = match utils::logging::setup_logging() {
        Ok((guard, ())) => (guard, ()),
        Err(e) => {
            tracing::error!("[-] Error Setting Up Logging: {:?}", e);
            panic!("[!] Error Setting Up Logging - Aborting")
        }
    };

    let pool = match db::establish_connection().await {
        Ok(pool) => pool,
        Err(e) => {
            tracing::error!("[-] Error Connecting to Database: {:?}", e);
            panic!("[!] Error Connecting to Database - Aborting")
        }
    };

    let redis = match cache::redis::create_redis_pool().await {
        Ok(redis) => redis,
        Err(e) => {
            tracing::error!("[-] Error Connecting to Redis: {:?}", e);
            panic!("[!] Error Connecting to Redis - Aborting")
        }
    };

    let system = sysinfo::System::new_all();

    // Build Rocket Instance
    rocket::build()
        .manage(pool)
        .manage(redis)
        .manage(system)
        .attach(fairings::cors::Cors)
        .attach(fairings::logging::Logging)
        .attach(AdHoc::on_shutdown("[!] Write Logs", |_| {
            Box::pin(async move { drop(guard) })
        }))
        .mount("/", routes![api::index::index])
        .mount(
            "/health",
            routes![api::health::redis, api::health::postgres],
        )
        .mount(
            "/system",
            routes![
                api::system::rust,
                api::system::package,
                api::system::git,
                api::system::system
            ],
        )
        .mount(
            "/v1/users",
            routes![
                api::users::get,
                api::users::create,
                api::users::update,
                api::users::delete,
                api::users::all,
                api::users::me
            ],
        )
        .register(
            "/",
            catchers![
                errors::default,
                errors::not_found,
                errors::internal_server_error
            ],
        )
}
