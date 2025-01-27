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

// Argon
extern crate argon2;

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
shadow_rs::shadow!(build);

// Launch Rocket Instance
#[launch]

async fn protein() -> _ {
    // Logging
    let (guard, ()) = match utils::logging::setup_logging() {
        Ok((guard, ())) => (guard, ()),
        Err(e) => {
            tracing::error!("[-] ❌ Error Setting Up Logging: {:?}", e);
            std::process::exit(1)
        }
    };

    // PostgreSQL
    let pool = match db::establish_connection().await {
        Ok(pool) => pool,
        Err(e) => {
            tracing::error!("[-] ❌ Error Connecting to Database: {:?}", e);
            std::process::exit(1)
        }
    };

    // Redis
    let redis = match cache::redis::create_redis_pool().await {
        Ok(redis) => redis,
        Err(e) => {
            tracing::error!("[-] ❌ Error Connecting to Redis: {:?}", e);
            std::process::exit(1)
        }
    };

    // System Information
    let system = sysinfo::System::new_all();

    // Rocket
    rocket::build()
        .manage(pool)
        .manage(redis)
        .manage(system)
        .attach(fairings::cors::Cors)
        .attach(fairings::logging::Logging)
        .attach(rocket_sentry::RocketSentry::fairing())
        .attach(rocket::fairing::AdHoc::on_shutdown(
            "[!] Write Logs",
            |_| Box::pin(async move { drop(guard) }),
        ))
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
                api::users::signup,
                api::users::login,
                api::users::update,
                api::users::update_password,
                api::users::delete,
                api::users::all,
                api::users::me
            ],
        )
        .mount(
            "/v1/keys",
            routes![
                api::keys::get,
                api::keys::all,
                api::keys::update,
                api::keys::revoke,
                api::keys::delete
            ],
        )
        .mount(
            "/v1/workouts",
            routes![
                api::workouts::get,
                api::workouts::all,
                api::workouts::update,
                api::workouts::create,
                api::workouts::delete
            ],
        )
        .mount(
            "/v1/calories",
            routes![
                api::calories::get,
                api::calories::all,
                api::calories::update,
                api::calories::create,
                api::calories::delete
            ],
        )
        .mount(
            "/v1/protein",
            routes![
                api::protein::get,
                api::protein::all,
                api::protein::update,
                api::protein::create,
                api::protein::delete
            ],
        )
        .mount(
            "/v1/sleep",
            routes![
                api::sleep::get,
                api::sleep::all,
                api::sleep::update,
                api::sleep::create,
                api::sleep::delete
            ],
        )
        .mount(
            "/v1/water",
            routes![
                api::water::get,
                api::water::all,
                api::water::update,
                api::water::create,
                api::water::delete
            ],
        )
        .mount(
            "/v1/exercises",
            routes![
                api::exercises::get,
                api::exercises::all,
                api::exercises::update,
                api::exercises::create,
                api::exercises::delete
            ],
        )
        .mount(
            "/v1/profile",
            routes![
                api::profile::get,
                api::profile::all,
                api::profile::create,
                api::profile::update,
                api::profile::delete
            ],
        )
        .register(
            "/",
            catchers![
                errors::default,
                errors::not_found,
                errors::internal_server_error,
                errors::unprocessable_entity,
                rocket_governor::rocket_governor_catcher
            ],
        )
}
