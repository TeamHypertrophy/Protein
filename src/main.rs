// Rocket Macro
#[macro_use] extern crate rocket;

// Rocket
// use rocket::fairing;
// use rocket::fairing::AdHoc;

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

// Migrations

//pub const MIGRATIONS: diesel_async_migrations::EmbeddedMigrations = diesel_async_migrations::embed_migrations!();

// pub async fn run_database_migrations(conn: diesel_async::AsyncPgConnection) -> fairing::Result {
//     // MIGRATIONS.run_pending_migrations(&mut conn).await.unwrap();
// }

// Launch Rocket Instance
#[launch]
async fn protein() -> _ {
    let pool = db::establish_connection().await.ok().unwrap();
    let redis = cache::redis::build_redis().await.ok().unwrap();
    //let conn = pool.get().await.expect("[!] Unable To Create Connection for Database Migrations");

    rocket::build()
        .manage(pool)
        .manage(redis)
        .attach(fairings::cors::Cors)
        //.attach(AdHoc::on_ignite("[!] Database Migrations", run_database_migrations(conn)))
        .mount("/", routes![api::index::index])
        .mount("/health", routes![api::health::api, api::health::redis, api::health::postgres])
        .mount("/system", routes![api::system::rust, api::system::package, api::system::git])
        .mount("/v1/users", routes![api::users::get, api::users::all])
        .register("/", catchers![errors::default, errors::not_found])
}
