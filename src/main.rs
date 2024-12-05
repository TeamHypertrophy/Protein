// Rocket Boilerplate
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

// Launch Rocket Instance
#[launch]
async fn protein() -> _ {
    let pool = db::establish_connection().await.ok().unwrap();

    rocket::build()
        .manage(pool)
        .attach(fairings::cors::Cors)
        .mount("/", routes![api::index::index, api::index::cat])
        .mount("/users", routes![api::users::get])
        .register("/", catchers![errors::default, errors::not_found])
}
