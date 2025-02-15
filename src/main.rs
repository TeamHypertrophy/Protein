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

// Vendor Dependencies
use sysinfo::System;
use tokio_cron_scheduler::{Job, JobScheduler};
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
#[tokio::main]
async fn protein() -> _ {
    // Load Environment Variables
    match dotenvy::from_filename(".env") {
        Ok(_) => {
            tracing::info!("[+] ✅ Environment Variables Loaded!");
        }
        Err(e) => {
            tracing::error!("[-] ❌ Error Loading Environment Variables: {:?}", e);
            std::process::exit(1)
        }
    }

    // Logging
    let (guard, ()) = match utils::logging::setup_logging() {
        Ok((guard, ())) => {
            tracing::info!("[+] ✅ Logging System Initialized!");
            (guard, ())
        }
        Err(e) => {
            tracing::error!("[-] ❌ Error Setting Up Logging: {:?}", e);
            std::process::exit(1)
        }
    };

    // PostgreSQL
    let pool = match db::establish_connection().await {
        Ok(pool) => {
            tracing::info!("[+] ✅ Database Connection Established!");
            pool
        }
        Err(e) => {
            tracing::error!("[-] ❌ Error Connecting to Database: {:?}", e);
            std::process::exit(1)
        }
    };

    // Redis
    let redis = match cache::redis::create_redis_pool().await {
        Ok(redis) => {
            tracing::info!("[+] ✅ Redis Connection Established!");
            redis
        }
        Err(e) => {
            tracing::error!("[-] ❌ Error Connecting to Redis: {:?}", e);
            std::process::exit(1)
        }
    };

    // Email
    let email = match utils::email::setup_email().await {
        Ok(email) => {
            tracing::info!("[+] ✅ Email System Initialized!");
            email
        }
        Err(e) => {
            tracing::error!("[-] ❌ Error Setting Up Email System: {:?}", e);
            std::process::exit(1)
        }
    };

    // Job Scheduler
    let scheduler = match JobScheduler::new().await {
        Ok(mut scheduler) => {
            tracing::info!("[+] ✅ Job Scheduler Initialized!");

            scheduler.shutdown_on_ctrl_c();

            scheduler.set_shutdown_handler(Box::new(|| {
                Box::pin(async move {
                    tracing::info!("[-] ❌ Job Scheduler Shutting Down!");
                })
            }));

            let cloned = pool.clone();

            let mut job = match Job::new_async("1/2 * * * * *", move |uuid, mut l| {
                Box::pin({
                    let val = cloned.clone();
                    async move {
                        println!("hello");
                        utils::jobs::verify_api_keys(&val).await.unwrap();
                    }
                })
            }) {
                Ok(job) => {
                    tracing::info!("[Scheduler] ✅ Job Created!");
                    job
                }
                Err(e) => {
                    tracing::error!("[Scheduler] ❌ Error Creating Job: {:?}", e);
                    std::process::exit(1)
                }
            };

            match scheduler.add(job).await {
                Ok(_) => tracing::info!("[Scheduler] ✅ Job Added To Scheduler!"),
                Err(e) => {
                    tracing::error!("[Scheduler] ❌ Error Adding Job To Scheduler: {:?}", e);
                    std::process::exit(1)
                }
            }
            scheduler
        }
        Err(e) => {
            tracing::error!("[-] ❌ Error Setting Up Job Scheduler: {:?}", e);
            std::process::exit(1)
        }
    };

    match scheduler.start().await {
        Ok(_) => tracing::info!("[Scheduler] ✅ Job Scheduler Started!"),
        Err(e) => {
            tracing::error!("[Scheduler] ❌ Error Starting Job Scheduler: {:?}", e);
            std::process::exit(1)
        }
    }

    // System Information
    let system: System = System::new_all();

    // Prometheus
    let prometheus = rocket_prometheus::PrometheusMetrics::new();

    // Rocket
    rocket::build()
        .manage(pool)
        .manage(redis)
        .manage(email)
        .manage(system)
        .manage(scheduler)
        .attach(prometheus.clone())
        .attach(fairings::cors::Cors)
        .attach(fairings::logging::Logging)
        .attach(rocket_sentry::RocketSentry::fairing())
        .attach(rocket::fairing::AdHoc::on_shutdown(
            "[!] Write Logs",
            |_| Box::pin(async move { drop(guard) }),
        ))
        .mount("/", routes![api::index::index])
        .mount("/metrics", prometheus)
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
                api::users::forgot_password_email,
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
                api::keys::delete
            ],
        )
        .mount(
            "/v1/workouts",
            routes![
                api::workouts::get,
                api::workouts::all,
                api::workouts::user_all,
                api::workouts::update,
                api::workouts::create,
                api::workouts::delete
            ],
        )
        .mount(
            "/v1/workout/plans",
            routes![
                api::plan::get,
                api::plan::all,
                api::plan::user_all,
                api::plan::update,
                api::plan::create,
                api::plan::delete
            ],
        )
        .mount(
            "/v1/trainers",
            routes![
                api::trainer::get,
                api::trainer::all,
                api::trainer::update,
                api::trainer::create,
                api::trainer::delete,
                api::trainer::get_announcement,
                api::trainer::all_announcements,
                api::trainer::all_trainer_announcements,
                api::trainer::update_announcement,
                api::trainer::create_announcement,
                api::trainer::delete_announcement
            ],
        )
        .mount(
            "/v1/calories",
            routes![
                api::calories::get,
                api::calories::all,
                api::calories::user_all,
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
                api::protein::user_all,
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
                api::sleep::user_all,
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
                api::water::user_all,
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
        .mount(
            "/v1/logs",
            routes![
                api::logs::exercise_get,
                api::logs::exercise_all,
                api::logs::exercise_user_all,
                api::logs::exercise_update,
                api::logs::exercise_create,
                api::logs::exercise_delete,
                api::logs::workout_get,
                api::logs::workout_all,
                api::logs::workout_user_all,
                api::logs::workout_update,
                api::logs::workout_create,
                api::logs::workout_delete
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
