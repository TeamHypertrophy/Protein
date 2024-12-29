/*
______          _       _       
| ___ \        | |     (_)      
| |_/ / __ ___ | |_ ___ _ _ __  
|  __/ '__/ _ \| __/ _ \ | '_ \ 
| |  | | | (_) | ||  __/ | | | |
\_|  |_|  \___/ \__\___|_|_| |_|

        Made with ❤️ 
*/
// Rocket
use rocket::State;
use rocket::serde::json;
use rocket::serde::json::Value;
use rocket::serde::{Serialize, Deserialize};

// Fred
use fred::prelude::*;

// .env Loader
use dotenvy::dotenv;
use std::env;

// STD
use std::time::Duration;

use crate::constants::CACHE_EXPIRATION_TIME;

pub type RedisPool = Pool;

pub async fn create_redis_pool() -> Result<Pool, Error> {
    // -- Load .env
    dotenv().ok();

    // -- Get Redis URI
    let redis_uri = env::var("REDIS_URI").expect("[!] REDIS_URI Environment Variable Must Be Set");

    // Create Redis Config
    let config = Config::from_url(&redis_uri).unwrap();
    let pool = Builder::from_config(config)
        .with_connection_config(|config| {
            config.connection_timeout = Duration::from_secs(10);
            config.tcp = TcpConfig {
                nodelay: Some(true),
                ..Default::default()
            };
        })
        .build_pool(5)
        .expect("[!] Failed to Create Redis Pool");

    pool
        .init()
        .await
        .expect("[!!] Failed to Initialize Redis Pool");

    Ok(pool)
}

// -- Redis Functions
pub struct Cache;

impl Cache {
    pub async fn get(pool: &State<RedisPool>, key: String) -> Result<Value, Error> {
        pool
            .get(key)
            .await
    }

    pub async fn set(pool: &State<RedisPool>, key: String, value: String) -> Result<(), Error> {
        pool
            .set(
                key,
                value,
                Some(Expiration::EX(CACHE_EXPIRATION_TIME)),
                None,
                false
            )
            .await
    }

    pub fn serialize<T: Serialize>(data: &T) -> String {
        json::to_string(data).unwrap()
    }

    pub fn deserialize<T: for<'de> Deserialize<'de>>(data: Value) -> T {
        json::from_value(data).unwrap()
    }
}