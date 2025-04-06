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

// Password Hashing
extern crate argon2;

// Vendor Dependencies
use rocket::fs::{FileServer, relative};
use tokio_cron_scheduler::{Job, JobScheduler};
use discord_webhook2::webhook::DiscordWebhook;
use user_agent_parser::UserAgentParser;

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
pub mod catchers;

// Constants
pub mod constants;

// Utils
pub mod utils;

// Responders
pub mod auth;
pub mod errors;

// Launch Rocket Instance
#[launch]
async fn protein() -> _ {
    // Load Environment Variables
    match dotenvy::from_filename(constants::ENV_PATH) {
        Ok(_) => {
            tracing::info!("[+] ✅ Environment Variables Loaded!");
        }
        Err(error) => {
            tracing::error!("[-] ❌ Error Loading Environment Variables: {:?}", error);
            std::process::exit(1)
        }
    }

    // Logging
    let (guard, ()) = match utils::logging::setup() {
        Ok((guard, ())) => {
            tracing::info!("[+] ✅ Logging System Initialized!");
            (guard, ())
        }
        Err(error) => {
            tracing::error!("[-] ❌ Error Setting Up Logging: {:?}", error);
            std::process::exit(1)
        }
    };

    // PostgreSQL
    let pool = match db::create().await {
        Ok(pool) => {
            tracing::info!("[+] ✅ Database Connection Established!");
            pool
        }
        Err(error) => {
            tracing::error!("[-] ❌ Error Connecting to Database: {:?}", error);
            std::process::exit(1)
        }
    };

    // Redis
    let redis = match cache::redis::create().await {
        Ok(redis) => {
            tracing::info!("[+] ✅ Redis Connection Established!");
            redis
        }
        Err(error) => {
            tracing::error!("[-] ❌ Error Connecting to Redis: {:?}", error);
            std::process::exit(1)
        }
    };

    // Email
    let email = match utils::email::setup().await {
        Ok(email) => {
            tracing::info!("[+] ✅ Email System Initialized!");
            email
        }
        Err(error) => {
            tracing::error!("[-] ❌ Error Setting Up Email System: {:?}", error);
            std::process::exit(1)
        }
    };

    // User Agent Parser
    let user_agent_parser = match UserAgentParser::from_path(constants::USER_AGENT_PARSER_PATH) {
        Ok(parser) => {
            tracing::info!("[+] ✅ User Agent Parser Initialized!");
            parser
        }
        Err(error) => {
            tracing::error!("[-] ❌ Error Initializing User Agent Parser: {:?}", error);
            std::process::exit(1)
        }
    };

    // Discord Webhooks
    let webhook = match DiscordWebhook::new(
        std::env::var("DISCORD_WEBHOOK_URL")
            .expect("[!] DISCORD_WEBHOOK_URL Environment Variable Must Be Set"),
    ) {
        Ok(webhook) => {
            tracing::info!("[+] ✅ Discord Webhook Initialized!");
            std::sync::Arc::new(webhook)
        }
        Err(error) => {
            tracing::error!("[-] ❌ Error Initializing Discord Webhook: {:?}", error);
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

            let database = pool.clone();

            let job = match Job::new_async(constants::API_KEY_JOB_INTERVAL, move |_uuid, _l| {
                Box::pin({
                    let db = database.clone();
                    async move {
                        match utils::jobs::verify_api_keys(&db).await {
                            Ok(_) => tracing::info!("[Scheduler] ✅ API Keys Verified!"),
                            Err(error) => tracing::error!(
                                "[Scheduler] ❌ Error Verifying API Keys: {:?}",
                                error
                            ),
                        }
                    }
                })
            }) {
                Ok(job) => {
                    tracing::info!("[Scheduler] ✅ API Key Job Created!");
                    job
                }
                Err(error) => {
                    tracing::error!("[Scheduler] ❌ Error Creating API Key Job: {:?}", error);
                    std::process::exit(1)
                }
            };

            let cleanup =
                match Job::new_async(constants::ASSET_CLEANER_JOB_INTERVAL, move |_uuid, _l| {
                    Box::pin({
                        async move {
                            match utils::jobs::clean_assets_directory().await {
                                Ok(_) => tracing::info!("[Scheduler] ✅ Assets Directory Cleaned!"),
                                Err(error) => tracing::error!(
                                    "[Scheduler] ❌ Error Cleaning Up Assets Directory: {:?}",
                                    error
                                ),
                            }
                        }
                    })
                }) {
                    Ok(job) => {
                        tracing::info!("[Scheduler] ✅ Assets Cleaner Job Created!");
                        job
                    }
                    Err(error) => {
                        tracing::error!(
                            "[Scheduler] ❌ Error Creating Asset Cleaner Job: {:?}",
                            error
                        );
                        std::process::exit(1)
                    }
                };

            match scheduler.add(job).await {
                Ok(_) => tracing::info!("[Scheduler] ✅ API Key Job Added To Scheduler!"),
                Err(error) => {
                    tracing::error!(
                        "[Scheduler] ❌ Error Adding API Key Job To Scheduler: {:?}",
                        error
                    );
                    std::process::exit(1)
                }
            }

            match scheduler.add(cleanup).await {
                Ok(_) => tracing::info!("[Scheduler] ✅ Cleanup Job Added To Scheduler!"),
                Err(error) => {
                    tracing::error!(
                        "[Scheduler] ❌ Error Adding Cleanup Job To Scheduler: {:?}",
                        error
                    );
                    std::process::exit(1)
                }
            }

            scheduler
        }
        Err(error) => {
            tracing::error!("[-] ❌ Error Setting Up Job Scheduler: {:?}", error);
            std::process::exit(1)
        }
    };

    // Start Job Scheduler
    match scheduler.start().await {
        Ok(_) => tracing::info!("[Scheduler] ✅ Job Scheduler Started!"),
        Err(error) => {
            tracing::error!("[Scheduler] ❌ Error Starting Job Scheduler: {:?}", error);
            std::process::exit(1)
        }
    }

    // Create Assets & Avatar Directory
    match async_fs::create_dir_all(constants::ASSETS_AVATARS_PATH).await {
        Ok(_) => tracing::info!("[+] ✅ Assets & Avatar Directory Created!"),
        Err(error) => {
            tracing::error!("[-] ❌ Error Creating Avatar Directory: {:?}", error);
            std::process::exit(1)
        }
    }

    // Prometheus Metrics
    let prometheus = rocket_prometheus::PrometheusMetrics::new();

    // Rocket
    rocket::build()
        .manage(pool)
        .manage(redis)
        .manage(email)
        .manage(scheduler)
        .manage(webhook)
        .manage(user_agent_parser)
        .manage(std::sync::Arc::new(utils::admin::Admin {
            routes: std::sync::LazyLock::new(|| {
                vec![
                    // Calorie Logs
                    "get_all_calorie_logs",
                    // Exercises
                    "update_exercise",
                    "create_exercise",
                    "delete_exercise",
                    // API Keys
                    "get_api_key",
                    "get_all_api_keys",
                    "get_all_user_api_keys",
                    "update_api_key",
                    "create_api_key",
                    "delete_api_key",
                    "revoke_api_key",
                    "change_api_key_role",
                    // Exercise & Workout Logs
                    "get_all_exercise_logs",
                    "get_all_workout_logs",
                    // Workout Plans
                    "get_all_workout_plans",
                    // Profile
                    "get_all_profiles",
                    "delete_profile",
                    // Protein Logs
                    "get_all_protein_logs",
                    // Sleep Logs
                    "get_all_sleep_logs",
                    // Trainers
                    "create_trainer",
                    // Users
                    "get_all_users",
                    "forgot_password_email",
                    // Water Logs
                    "get_all_water_logs",
                    // Workouts
                    "get_all_workouts",
                    // Workout Plan Logs
                    "get_all_workout_plan_logs",
                ]
            }),
            master_key: std::env::var("MASTER_API_KEY")
                .expect("[!] MASTER_API_KEY Environment Variable Must Be Set"),
            app_env: std::env::var("APP_ENV")
                .expect("[!] APP_ENV Environment Variable Must Be Set"),
            host_url: std::env::var("HOST_URL")
                .expect("[!] HOST_URL Environment Variable Must Be Set"),
            avatar_host_url: std::env::var("AVATAR_HOST_URL")
                .expect("[!] AVATAR_HOST_URL Environment Variable Must Be Set"),
            password_salt: std::env::var("PASSWORD_SALT")
                .expect("[!] PASSWORD_SALT Environment Variable Must Be Set"),
            smtp_username: std::env::var("SMTP_USERNAME")
                .expect("[!] SMTP_USERNAME Environment Variable Must Be Set"),
            smtp_user: std::env::var("SMTP_USER")
                .expect("[!] SMTP_USER Environment Variable Must Be Set"),
        }))
        .attach(prometheus.clone())
        .attach(fairings::cors::Cors) // Adds CORS (Cross-Origin Resource Sharing) Headers
        .attach(rocket_governor::LimitHeaderGen) // Generates X_RATELIMIT_LIMIT and X_RATELIMIT_REMAINING Headers
        .attach(rocket_sentry::RocketSentry::fairing()) // Sets Up Sentry Reporting on panic!() calls
        .attach(rocket::fairing::AdHoc::on_shutdown(
            "[!] Write Logs",
            |_| Box::pin(async move { drop(guard) }), /* guard is dropped to ensure log files
                                                       * are written on server exit */
        ))
        .mount("/", routes![api::index::index])
        .mount("/assets", FileServer::from(relative!("assets")))
        .mount("/metrics", prometheus)
        .mount(
            "/v1/health",
            routes![
                api::health::ping_redis,
                api::health::ping_postgres,
                api::health::get_version
            ],
        )
        .mount(
            "/v1/users",
            routes![
                api::users::get_user,
                api::users::signup,
                api::users::login,
                api::users::update_user,
                api::users::update_user_password,
                api::users::verify_email,
                api::users::request_password_reset,
                api::users::password_request_check_code,
                api::users::reset_password,
                api::users::delete_user,
                api::users::get_all_users,
                api::users::enable_mfa,
                api::users::check_mfa,
                api::users::verify_mfa,
                api::users::disable_mfa,
                api::users::resend_mfa,
            ],
        )
        .mount(
            "/v1/keys",
            routes![
                api::keys::get_api_key,
                api::keys::create_api_key,
                api::keys::get_all_api_keys,
                api::keys::get_all_user_api_keys,
                api::keys::update_api_key,
                api::keys::delete_api_key,
                api::keys::revoke_api_key,
                api::keys::change_api_key_role,
                api::keys::is_admin_key,
            ],
        )
        .mount(
            "/v1/workouts",
            routes![
                api::workouts::get_workout,
                api::workouts::get_all_workouts,
                api::workouts::get_all_user_workouts,
                api::workouts::update_workout,
                api::workouts::create_workout,
                api::workouts::delete_workout
            ],
        )
        .mount(
            "/v1/workout/plans",
            routes![
                api::plan::get_workout_plan,
                api::plan::get_all_workout_plans,
                api::plan::get_all_user_workout_plans,
                api::plan::update_workout_plan,
                api::plan::create_workout_plan,
                api::plan::delete_workout_plan,
                api::plan::get_workout_plan_log,
                api::plan::get_all_workout_plan_logs,
                api::plan::get_all_user_workout_plan_logs,
                api::plan::update_workout_plan_log,
                api::plan::create_workout_plan_log,
                api::plan::delete_workout_plan_log,
            ],
        )
        .mount(
            "/v1/trainers",
            routes![
                api::trainer::get_trainer,
                api::trainer::get_all_trainers,
                api::trainer::update_trainer,
                api::trainer::create_trainer,
                api::trainer::delete_trainer,
                api::trainer::get_announcement,
                api::trainer::get_all_announcements,
                api::trainer::get_all_trainer_announcements,
                api::trainer::update_announcement,
                api::trainer::create_announcement,
                api::trainer::delete_announcement
            ],
        )
        .mount(
            "/v1/calories",
            routes![
                api::calories::get_calorie_log,
                api::calories::get_all_calorie_logs,
                api::calories::get_all_user_calorie_logs,
                api::calories::update_calorie_log,
                api::calories::create_calorie_log,
                api::calories::delete_calorie_log
            ],
        )
        .mount(
            "/v1/protein",
            routes![
                api::protein::get_protein_log,
                api::protein::get_all_protein_logs,
                api::protein::get_all_user_protein_logs,
                api::protein::update_protein_log,
                api::protein::create_protein_log,
                api::protein::delete_protein_log
            ],
        )
        .mount(
            "/v1/sleep",
            routes![
                api::sleep::get_sleep_log,
                api::sleep::get_all_sleep_logs,
                api::sleep::get_all_user_sleep_logs,
                api::sleep::update_sleep_log,
                api::sleep::create_sleep_log,
                api::sleep::delete_sleep_log
            ],
        )
        .mount(
            "/v1/water",
            routes![
                api::water::get_water_log,
                api::water::get_all_water_logs,
                api::water::get_all_user_water_logs,
                api::water::update_water_log,
                api::water::create_water_log,
                api::water::delete_water_log
            ],
        )
        .mount(
            "/v1/exercises",
            routes![
                api::exercises::get_exercise,
                api::exercises::get_all_exercises,
                api::exercises::search_exercises,
                api::exercises::update_exercise,
                api::exercises::create_exercise,
                api::exercises::delete_exercise
            ],
        )
        .mount(
            "/v1/exercises/custom",
            routes![
                api::custom::get_custom_exercise,
                api::custom::get_all_custom_exercises,
                api::custom::get_all_user_custom_exercises,
                api::custom::update_custom_exercise,
                api::custom::create_custom_exercise,
                api::custom::delete_custom_exercise
            ],
        )
        .mount(
            "/v1/profile",
            routes![
                api::profile::get_profile,
                api::profile::get_all_profiles,
                api::profile::create_profile,
                api::profile::update_profile,
                api::profile::upload_avatar,
                api::profile::delete_profile
            ],
        )
        .mount(
            "/v1/logs",
            routes![
                api::logs::get_exercise_log,
                api::logs::get_all_exercise_logs,
                api::logs::get_all_user_exercise_logs,
                api::logs::update_exercise_log,
                api::logs::create_exercise_log,
                api::logs::delete_exercise_log,
                api::logs::get_workout_log,
                api::logs::get_all_workout_logs,
                api::logs::get_all_user_workout_logs,
                api::logs::update_workout_log,
                api::logs::create_workout_log,
                api::logs::delete_workout_log
            ],
        )
        .register(
            "/",
            catchers![
                catchers::default,
                catchers::not_found,
                catchers::internal_server_error,
                catchers::unprocessable_entity,
                catchers::unauthorized,
                rocket_governor::rocket_governor_catcher
            ],
        )
}
