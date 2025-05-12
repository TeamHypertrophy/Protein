/*
______          _       _
| ___ \        | |     (_)
| |_/ / __ ___ | |_ ___ _ _ __
|  __/ '__/ _ \| __/ _ \ | '_ \
| |  | | | (_) | ||  __/ | | | |
\_|  |_|  \___/ \__\___|_|_| |_|

        Made with ❤️
*/

use std::{fmt::Display, time::Duration};

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

    // Create Redis Pool
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
                nodelay: Some(CACHE_TCP_CONFIG_DELAY),
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
    // Get Value From Redis Cache
    pub async fn get<T: Display>(pool: &State<Redis>, group: &str, key: T) -> Result<Value, Error> {
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

    // Sets A Value In Redis Cache
    pub async fn set<T: Display>(
        pool: &State<Redis>,
        group: &str,
        key: T,
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

    pub async fn job_set<T: Display>(
        pool: &Redis,
        group: &str,
        key: T,
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

    // Deletes A Value From Redis Cache
    pub async fn delete<T: Display>(pool: &State<Redis>, group: &str, key: T) -> Result<(), Error> {
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

    // Pings Redis For Health Check
    pub async fn ping(pool: &State<Redis>, message: Option<String>) -> Result<String, Error> {
        tracing::info!("[Cache] ⚙️ Pinging Redis For Health Check");

        pool.ping(message).await.map_err(|error| {
            tracing::error!("[!] Redis Error: {:?}", error);
            Error::Cache(error.to_string())
        })
    }

    // Checks If A Key Exists In Redis Cache
    pub async fn exists<T: Display>(pool: &Redis, group: &str, key: T) -> Result<bool, Error> {
        tracing::info!(
            "[Cache] ⚙️ Checking If Key Exists In Redis Cache: {:#?}",
            format!("{}:{}", group, key)
        );

        pool.exists(format!("{}:{}", group, key))
            .await
            .map_err(|error| {
                tracing::error!("[!] Redis Error: {:?}", error);
                Error::Cache(error.to_string())
            })
    }

    // Serializes Data To JSON
    pub fn serialize<T: Serialize>(data: &T) -> Result<String, Error> {
        json::to_string(data).map_err(|error| Error::Cache(error.to_string()))
    }

    // Deserializes Data From JSON
    pub fn deserialize<T: for<'de> Deserialize<'de>>(data: Value) -> Result<T, Error> {
        json::from_value(data).map_err(|error| Error::Cache(error.to_string()))
    }
}
