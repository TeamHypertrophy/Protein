/*
______          _       _
| ___ \        | |     (_)
| |_/ / __ ___ | |_ ___ _ _ __
|  __/ '__/ _ \| __/ _ \ | '_ \
| |  | | | (_) | ||  __/ | | | |
\_|  |_|  \___/ \__\___|_|_| |_|

        Made with ❤️
*/

use std::{env, time::Duration};

use rocket::{
    serde::{json, json::Value, Deserialize, Serialize},
    State,
};
use fred::prelude::*;
use dotenvy::dotenv;

use crate::{constants::CACHE_EXPIRATION_TIME, responders::ProteinError};

pub type RedisPool = Pool;

pub async fn create_redis_pool() -> Result<Pool, Error> {
    // Load .env
    dotenv().ok();

    // Get Redis URI
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

    // Initialize Pool
    pool.init()
        .await
        .expect("[!!] Failed to Initialize Redis Pool");

    Ok(pool)
}

// -- Redis Functions
pub struct Cache;

impl Cache {
    pub async fn get(
        pool: &State<RedisPool>,
        group: &str,
        key: String,
    ) -> Result<Value, ProteinError> {
        tracing::info!(
            "[>] Fetching Key From Redis Cache: {}",
            format!("{}:{}", group, key)
        );

        pool.get(format!("{}:{}", group, key))
            .await
            .map_err(|error| {
                tracing::error!("[!] Redis Error: {:?}", error);
                ProteinError::Cache(error.to_string())
            })
    }

    pub async fn set(
        pool: &State<RedisPool>,
        group: &str,
        key: String,
        value: String,
    ) -> Result<(), ProteinError> {
        tracing::info!(
            "[>] Setting Key In Redis Cache: {} With Values: {}",
            format!("{}:{}", group, key),
            value
        );

        pool.set(
            format!("{}:{}", group, key),
            value,
            Some(Expiration::EX(CACHE_EXPIRATION_TIME)),
            None,
            false,
        )
        .await
        .map_err(|error| {
            tracing::error!("[!] Redis Error: {:?}", error);
            ProteinError::Cache(error.to_string())
        })
    }

    pub async fn ping(
        pool: &State<RedisPool>,
        message: Option<String>,
    ) -> Result<String, ProteinError> {
        tracing::info!("[>] Pinging Redis For Health Check");

        pool.ping(message).await.map_err(|error| {
            tracing::error!("[!] Redis Error: {:?}", error);
            ProteinError::Cache(error.to_string())
        })
    }

    pub fn serialize<T: Serialize>(data: &T) -> String {
        json::to_string(data).unwrap()
    }

    pub fn deserialize<T: for<'de> Deserialize<'de>>(data: Value) -> T {
        json::from_value(data).unwrap()
    }
}
