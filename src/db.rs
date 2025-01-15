/*
______          _       _
| ___ \        | |     (_)
| |_/ / __ ___ | |_ ___ _ _ __
|  __/ '__/ _ \| __/ _ \ | '_ \
| |  | | | (_) | ||  __/ | | | |
\_|  |_|  \___/ \__\___|_|_| |_|

        Made with ❤️
*/

use rocket::{tokio, State};
use diesel_async::{
    async_connection_wrapper::AsyncConnectionWrapper,
    pg::AsyncPgConnection,
    pooled_connection::{deadpool, deadpool::Pool, AsyncDieselConnectionManager},
};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use crate::responders::ProteinError;
use dotenvy::dotenv;
use std::env;

pub type DatabaseConnection = deadpool::Object<AsyncPgConnection>;
pub type DatabasePool = Pool<AsyncPgConnection>;

pub async fn establish_connection() -> Result<DatabasePool, Box<dyn std::error::Error>> {
    // Load .env
    dotenv().ok();

    // Get Database URL
    let database_uri: String =
        env::var("DATABASE_URL").expect("[!] DATABASE_URL Environment Variable Must Be Set");

    // Create Manager
    let manager: AsyncDieselConnectionManager<AsyncPgConnection> =
        AsyncDieselConnectionManager::<AsyncPgConnection>::new(database_uri);

    // Create Database Pool
    let pool = Pool::builder(manager)
        .max_size(10)
        .build()
        .expect("[!] Could Not Create Database Pool");

    // Define Migrations
    pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

    // Get Database Connection
    let connection = pool.clone().get().await?;

    // Run Migrations
    let mut wrapper: AsyncConnectionWrapper<DatabaseConnection> =
        AsyncConnectionWrapper::from(connection);

    tokio::task::spawn_blocking(move || {
        wrapper.run_pending_migrations(MIGRATIONS).unwrap();
    })
    .await?;

    // Return Pool
    Ok(pool)
}

// Retrieves Single Database Connection
pub async fn get_connection(
    pool: &State<DatabasePool>,
) -> Result<DatabaseConnection, ProteinError> {
    pool.get().await.map_err(|error| {
        tracing::error!("[!] PostgreSQL Error {:?}", error);
        ProteinError::Database(error.to_string())
    })
}
