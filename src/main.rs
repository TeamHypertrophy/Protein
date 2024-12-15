// Rocket Macro
#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel_async_migrations;

// Rocket
use rocket::{Rocket, Build};
use rocket::fairing::AdHoc;

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

// Database Migrations

pub async fn run_database_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    pub const MIGRATIONS: diesel_async_migrations::EmbeddedMigrations = embed_migrations!("migrations");

    // Run Migrations
    //let pool = rocket.state::<db::DatabasePool>().unwrap();
    //let mut connection = pool.get().await.ok().unwrap();
    //MIGRATIONS.run_pending_migrations(&mut connection).await.unwrap();

    // Return Rocket<Build> Instance
    rocket
}

// Launch Rocket Instance
#[launch]
async fn protein() -> _ {
    let pool = db::establish_connection().await.ok().unwrap();
    let redis = cache::redis::build_redis().await.ok().unwrap();

    rocket::build()
        .manage(pool)
        .manage(redis)
        .attach(fairings::cors::Cors)
        .attach(AdHoc::on_ignite("[!] Database Migrations", run_database_migrations))
        .mount("/", routes![api::index::index])
        .mount("/health", routes![api::health::redis, api::health::postgres])
        .mount("/system", routes![api::system::rust, api::system::package, api::system::git])
        .mount("/v1/users", routes![api::users::get, api::users::all])
        .register("/", catchers![errors::default, errors::not_found])
}
