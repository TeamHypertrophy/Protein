// Rocket
//use rocket::tokio;

// Diesel Async
use diesel_async::pg::AsyncPgConnection;
//use diesel_async::async_connection_wrapper::AsyncConnectionWrapper;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::pooled_connection::bb8::Pool;

// Diesel Async Migrations
//use diesel_async_migrations::{EmbeddedMigrations, embed_migrations};

// .env Loader
use dotenvy::dotenv;
use std::env;

// Error Handling
//use anyhow::anyhow;

// Establish Database Connection Pool

pub type DatabasePool = Pool<AsyncPgConnection>;

pub async fn establish_connection() -> Result<DatabasePool, Box<dyn std::error::Error>> {
    // -- Load .env
    dotenv().ok();

    // -- Get Database URL
    let database_uri = env::var("DATABASE_URI").expect("[!] DATABASE_URI Environment Variable Must Be Set");

    // -- Create Manager
    let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(database_uri);

    // -- Create Database Pool
    let pool = Pool::builder()
        .max_size(10)
        .build(manager)
        .await
        .expect("[!] Could Not Create Database Pool");

    // -- Define Migrations
   // pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

    // -- Get Database Connection
    //let mut connection = pool.clone().get().await.ok().unwrap();

    // -- Run Migrations
  //  let wrapper = AsyncConnectionWrapper::from(connection);

    //tokio::task::spawn_blocking(move || {
      //  wrapper.run_pending_migrations(MIGRATIONS).unwrap();
   //	}).await?;

    // Return Pool
    Ok(pool)
}
