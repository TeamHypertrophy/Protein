// Diesel Imports
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use dotenvy::dotenv;
use std::env;


// Establish Database Connection Pool

pub type DatabasePool = Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> DatabasePool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("[!] DATABASE_URL Environment Variable Must Be Set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);

    Pool::builder()
        .build(manager)
        .expect("[!] Failed to Create Database Pool")
}