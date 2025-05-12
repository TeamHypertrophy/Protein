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

use crate::{constants, errors::Error};

pub type Conn = deadpool::Object<AsyncPgConnection>;
pub type DB = Pool<AsyncPgConnection>;

pub async fn create() -> Result<DB, Box<dyn std::error::Error>> {
    // Get Database URL
    let database_url: String =
        std::env::var("DATABASE_URL").expect("[!] DATABASE_URL Environment Variable Must Be Set");

    // Create Manager
    let manager: AsyncDieselConnectionManager<AsyncPgConnection> =
        AsyncDieselConnectionManager::<AsyncPgConnection>::new(database_url);

    // Create Database Pool
    let pool: DB = match Pool::builder(manager)
        .max_size(constants::POSTGRES_POOL_SIZE)
        .build()
    {
        Ok(pool) => pool,
        Err(error) => panic!("[!] Could Not Create Database Pool: {}", error),
    };

    // Define Migrations
    pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

    // Get Database Connection
    let connection: Conn = pool.clone().get().await?;

    // Run Migrations
    let mut wrapper: AsyncConnectionWrapper<Conn> = AsyncConnectionWrapper::from(connection);

    rocket::tokio::task::spawn_blocking(move || match wrapper.run_pending_migrations(MIGRATIONS) {
        Ok(_) => (),
        Err(error) => panic!("[!] Failed Running Database Migrations: {}", error),
    })
    .await?;

    // Return Pool
    Ok(pool)
}

#[inline]
pub async fn get(pool: &State<DB>) -> Result<Conn, Error> {
    pool.get().await.map_err(|error| {
        tracing::error!("[!] PostgreSQL Error {:?}", error);
        Error::Database(error.to_string())
    })
}
