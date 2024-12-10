// Rocket Macro
#[macro_use] extern crate rocket;

// Schema File
mod schema;

// Database
mod models;
mod db;

// API Routes
pub mod api;

// Fairings
pub mod fairings;

// Error Handlers
pub mod errors;

// Migrations
use db::DatabaseConnection;

pub const MIGRATIONS: diesel_async_migrations::EmbeddedMigrations = diesel_async_migrations::embed_migrations!();

pub async fn run_database_migrations(conn: &DatabaseConnection) -> () {
    // TODO!
}

// Launch Rocket Instance
#[launch]
async fn protein() -> _ {
    let pool = db::establish_connection().await.ok().unwrap();

    rocket::build()
        .manage(pool)
        .attach(fairings::cors::Cors)
        .mount("/", routes![api::index::index, api::index::cat])
        .mount("/v1/users", routes![api::users::get, api::users::all])
        .register("/", catchers![errors::default, errors::not_found])
}
