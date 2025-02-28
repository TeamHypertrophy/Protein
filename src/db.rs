/*
______          _       _
| ___ \        | |     (_)
| |_/ / __ ___ | |_ ___ _ _ __
|  __/ '__/ _ \| __/ _ \ | '_ \
| |  | | | (_) | ||  __/ | | | |
\_|  |_|  \___/ \__\___|_|_| |_|

        Made with ❤️
*/

use rocket::State;
use diesel_async::{
    async_connection_wrapper::AsyncConnectionWrapper,
    pg::AsyncPgConnection,
    pooled_connection::{AsyncDieselConnectionManager, deadpool, deadpool::Pool},
};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};

use crate::{constants::POSTGRES_POOL_SIZE, errors::Error};

pub type DBConnection = deadpool::Object<AsyncPgConnection>;
pub type DB = Pool<AsyncPgConnection>;

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
/// * `Result<DB, Box<dyn std::error::Error>>` - A Result containing either:
///   - `DB`: The configured connection pool
///   - `Box<dyn std::error::Error>`: Any error that occurred during setup
///
/// # Errors
/// Returns an error if:
/// * DATABASE_URL environment variable is not set
/// * Failed to create connection pool
/// * Failed to run migrations
/// * Database is unreachable
pub async fn create() -> Result<DB, Box<dyn std::error::Error>> {
    // Get Database URL
    let database_url: String =
        std::env::var("DATABASE_URL").expect("[!] DATABASE_URL Environment Variable Must Be Set");

    // Create Manager
    let manager: AsyncDieselConnectionManager<AsyncPgConnection> =
        AsyncDieselConnectionManager::<AsyncPgConnection>::new(database_url);

    // Create Database Pool
    let pool: DB = match Pool::builder(manager).max_size(POSTGRES_POOL_SIZE).build() {
        Ok(pool) => pool,
        Err(error) => panic!("[!] Could Not Create Database Pool: {}", error),
    };

    // Define Migrations
    pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

    // Get Database Connection
    let connection: DBConnection = pool.clone().get().await?;

    // Run Migrations
    let mut wrapper: AsyncConnectionWrapper<DBConnection> =
        AsyncConnectionWrapper::from(connection);

    rocket::tokio::task::spawn_blocking(move || match wrapper.run_pending_migrations(MIGRATIONS) {
        Ok(_) => (),
        Err(error) => panic!("[!] Failed Running Database Migrations: {}", error),
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
/// * `Result<DBConnection, Error>` - A Result containing either:
///   - `DBConnection`: A successful connection from the pool
///   - `Error`: A database error with details
///
/// # Example
/// ```
/// let connection = db::get_connection(pool).await?;
/// let users = User::all(&mut connection).await?;
/// ```
///
/// # Errors
/// Returns a `Error::Database` if:
/// * The pool is exhausted
/// * Connection timeout occurs
/// * Database is unreachable
pub async fn get_connection(pool: &State<DB>) -> Result<DBConnection, Error> {
    pool.get().await.map_err(|error| {
        tracing::error!("[!] PostgreSQL Error {:?}", error);
        Error::Database(error.to_string())
    })
}
