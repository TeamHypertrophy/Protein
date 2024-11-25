// Rocket Boilerplate
#[macro_use] extern crate rocket;

// Schema File
mod schema;

// Database
mod models;
mod db;

// API Routes
pub mod api;

// Error Handlers
pub mod errors;

// Launch Rocket Instance
#[launch]
async fn rocket() -> _ {
    let pool = db::establish_connection();

    rocket::build()
        .manage(pool)
        .mount("/", routes![api::index::index])
        .mount("/users", routes![api::users::get])
        .register("/", catchers![errors::default, errors::not_found])
}
