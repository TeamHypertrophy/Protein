/*
______          _       _
| ___ \        | |     (_)
| |_/ / __ ___ | |_ ___ _ _ __
|  __/ '__/ _ \| __/ _ \ | '_ \
| |  | | | (_) | ||  __/ | | | |
\_|  |_|  \___/ \__\___|_|_| |_|

        Made with ❤️
*/

use std::env;

use rocket::{tokio, State};
use diesel_async::{
    async_connection_wrapper::AsyncConnectionWrapper,
    pg::AsyncPgConnection,
    pooled_connection::{deadpool, deadpool::Pool, AsyncDieselConnectionManager},
};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenvy::dotenv;

use crate::{constants::POSTGRES_POOL_SIZE, responders::ProteinError};

pub type DatabaseConnection = deadpool::Object<AsyncPgConnection>;
pub type DatabasePool = Pool<AsyncPgConnection>;

/// Establishes a connection pool to the PostgreSQL database and runs
/// migrations.
///
/// This function:
/// 1. Loads environment variables from .env file
/// 2. Creates a connection pool using DATABASE_URL
/// 3. Runs any pending database migrations
/// 4. Returns the configured connection pool
///
/// # Environment Variables Required
/// * `DATABASE_URL` - PostgreSQL connection string
/// * `POSTGRES_POOL_SIZE` - Maximum number of connections in pool
///
/// # Returns
/// * `Result<DatabasePool, Box<dyn std::error::Error>>` - A Result containing
///   either:
///   - `DatabasePool`: The configured connection pool
///   - `Box<dyn std::error::Error>`: Any error that occurred during setup
///
/// # Errors
/// Returns an error if:
/// * DATABASE_URL environment variable is not set
/// * Failed to create connection pool
/// * Failed to run migrations
/// * Database is unreachable
pub async fn establish_connection() -> Result<DatabasePool, Box<dyn std::error::Error>> {
    // Load .env
    dotenv()?;

    // Get Database URL
    let database_url: String =
        env::var("DATABASE_URL").expect("[!] DATABASE_URL Environment Variable Must Be Set");

    // Create Manager
    let manager: AsyncDieselConnectionManager<AsyncPgConnection> =
        AsyncDieselConnectionManager::<AsyncPgConnection>::new(database_url);

    // Create Database Pool
    let pool: DatabasePool = Pool::builder(manager)
        .max_size(POSTGRES_POOL_SIZE)
        .build()
        .expect("[!] Could Not Create Database Pool");

    // Define Migrations
    pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

    // Get Database Connection
    let connection: DatabaseConnection = pool.clone().get().await?;

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

/// Retrieves a single database connection from the connection pool.
///
/// # Arguments
/// * `pool` - Reference to the database connection pool managed by Rocket's
///   state
///
/// # Returns
/// * `Result<DatabaseConnection, ProteinError>` - A Result containing either:
///   - `DatabaseConnection`: A successful connection from the pool
///   - `ProteinError`: A database error with details
///
/// # Example
/// ```
/// let connection = get_connection(pool).await?;
/// let users = User::all(&mut connection).await?;
/// ```
///
/// # Errors
/// Returns a `ProteinError::Database` if:
/// * The pool is exhausted
/// * Connection timeout occurs
/// * Database is unreachable
pub async fn get_connection(
    pool: &State<DatabasePool>,
) -> Result<DatabaseConnection, ProteinError> {
    pool.get().await.map_err(|error| {
        tracing::error!("[!] PostgreSQL Error {:?}", error);
        ProteinError::Database(error.to_string())
    })
}
