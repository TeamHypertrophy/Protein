/*
______          _       _
| ___ \        | |     (_)
| |_/ / __ ___ | |_ ___ _ _ __
|  __/ '__/ _ \| __/ _ \ | '_ \
| |  | | | (_) | ||  __/ | | | |
\_|  |_|  \___/ \__\___|_|_| |_|

        Made with ❤️
*/

use std::time::Duration;

use rocket::{
    State,
    serde::{Deserialize, Serialize, json, json::Value},
};
use fred::{prelude::*, types::config::UnresponsiveConfig};

use crate::{constants::*, errors::Error};

pub type Redis = Pool;

pub async fn create() -> Result<Pool, Error> {
    // Get Redis URI
    let redis_uri =
        std::env::var("REDIS_URI").expect("[!] REDIS_URI Environment Variable Must Be Set");

    // Create Redis Config
    let config = match Config::from_url(&redis_uri) {
        Ok(config) => config,
        Err(error) => panic!("[!] Failed To Create Redis Config: {}", error),
    };

    let pool = match Builder::from_config(config)
        .with_connection_config(|config| {
            config.connection_timeout = Duration::from_secs(CACHE_CONNECTION_TIMEOUT);
            config.max_command_attempts = CACHE_MAX_COMMAND_ATTEMPTS;
            config.max_redirections = CACHE_MAX_REDIRECTIONS;

            config.unresponsive = UnresponsiveConfig {
                max_timeout: Some(Duration::from_secs(CACHE_UNRESPONSIVE_MAX_TIMEOUT)),
                interval: Duration::from_secs(CACHE_UNRESPONSIVE_INTERVAL),
            };

            config.tcp = TcpConfig {
                nodelay: Some(true),
                ..Default::default()
            };
        })
        .set_policy(ReconnectPolicy::new_exponential(
            CACHE_RECONNECT_POLICY.0,
            CACHE_RECONNECT_POLICY.1,
            CACHE_RECONNECT_POLICY.2,
            CACHE_RECONNECT_POLICY.3,
        ))
        .build_pool(CACHE_POOL_SIZE)
    {
        Ok(pool) => pool,
        Err(error) => panic!("[!] Failed to Create Redis Pool: {}", error),
    };

    // Initialize Pool
    match pool.init().await {
        Ok(_) => (),
        Err(error) => panic!("[!!] Failed to Initialize Redis Pool: {}", error),
    }

    Ok(pool)
}

// -- Redis Functions
pub struct Cache;

impl Cache {
    pub async fn get(pool: &State<Redis>, group: &str, key: String) -> Result<Value, Error> {
        tracing::info!(
            "[Cache] ⚙️ Fetching Key From Redis Cache: {:#?}",
            format!("{}:{}", group, key)
        );

        pool.get(format!("{}:{}", group, key))
            .await
            .map_err(|error| {
                tracing::error!("[!] Redis Error: {:?}", error);
                Error::Cache(error.to_string())
            })
    }

    pub async fn set(
        pool: &State<Redis>,
        group: &str,
        key: String,
        value: String,
    ) -> Result<(), Error> {
        tracing::info!(
            "[Cache] ⚙️ Setting Key In Redis Cache: {:#?} With Values: {:#?}",
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
            Error::Cache(error.to_string())
        })
    }

    pub async fn delete(pool: &State<Redis>, group: &str, key: String) -> Result<(), Error> {
        tracing::info!(
            "[Cache] ⚙️ Deleting Key From Redis Cache: {:#?}",
            format!("{}:{}", group, key)
        );

        pool.del(format!("{}:{}", group, key))
            .await
            .map_err(|error| {
                tracing::error!("[!] Redis Error: {:?}", error);
                Error::Cache(error.to_string())
            })
    }

    pub async fn ping(pool: &State<Redis>, message: Option<String>) -> Result<String, Error> {
        tracing::info!("[Cache] ⚙️ Pinging Redis For Health Check");

        pool.ping(message).await.map_err(|error| {
            tracing::error!("[!] Redis Error: {:?}", error);
            Error::Cache(error.to_string())
        })
    }

    pub fn serialize<T: Serialize>(data: &T) -> Result<String, Error> {
        json::to_string(data).map_err(|error| Error::Cache(error.to_string()))
    }

    pub fn deserialize<T: for<'de> Deserialize<'de>>(data: Value) -> Result<T, Error> {
        json::from_value(data).map_err(|error| Error::Cache(error.to_string()))
    }
}
