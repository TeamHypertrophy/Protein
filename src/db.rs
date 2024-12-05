// Diesel Async
use diesel_async::AsyncPgConnection;
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
    let database_url = env::var("DATABASE_URL").expect("[!] DATABASE_URL Environment Variable Must Be Set");

    // -- Create Manager
    let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(database_url);

    // -- Create Pool and Return
    let pool = Pool::builder()
        .build(manager)
        .await?;

    Ok(pool)
}
