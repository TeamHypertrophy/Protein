// Diesel Async
use diesel_async::pg::AsyncPgConnection;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::pooled_connection::bb8::Pool;

// .env Loader
use dotenvy::dotenv;
use std::env;

// Establish Database Connection Pool

pub type DatabasePool = Pool<AsyncPgConnection>;

pub async fn establish_connection() -> Result<DatabasePool, Box<dyn std::error::Error>> {
    // -- Load .env
    dotenv().ok();

    // -- Get Database URL
    let database_uri = env::var("DATABASE_URI").expect("[!] DATABASE_URI Environment Variable Must Be Set");

    // -- Create Manager
    let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(database_uri);

    // -- Create Pool and Return
    let pool = Pool::builder()
        .max_size(10)
        .build(manager)
        .await
        .expect("[!] Could Not Create Database Pool");

    Ok(pool)
}
