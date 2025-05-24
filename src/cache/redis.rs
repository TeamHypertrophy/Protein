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

use uuid::Uuid;
use rocket::{
    State,
    serde::{Deserialize, Serialize, json, json::Value},
};
use fred::{prelude::*, types::config::UnresponsiveConfig};

use crate::{constants::*, errors::Error};

pub type Redis = Pool;

// -- Trait to Convert More Complex Types Into Cache Keys
pub trait CacheKey {
    fn get(&self) -> String;
}

impl CacheKey for &str {
    fn get(&self) -> String {
        self.to_string()
    }
}

impl CacheKey for String {
    fn get(&self) -> String {
        self.clone()
    }
}

impl CacheKey for Uuid {
    fn get(&self) -> String {
        self.to_string()
    }
}

impl<T: Display, U: Display> CacheKey for (T, U) {
    fn get(&self) -> String {
        format!("{}_{}", self.0, self.1)
    }
}

macro_rules! impl_cache_key_for_numbers {
    ($($t:ty),*) => {
        $(
            impl CacheKey for $t {
                fn get(&self) -> String { self.to_string() }
            }
            impl CacheKey for &$t {
                fn get(&self) -> String { (*self).to_string() }
            }
        )*
    };
}

impl_cache_key_for_numbers!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
);
impl_cache_key_for_numbers!(f32, f64);

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

// -- Cache Groups
// Type Safe Enum for Cache Groups
#[derive(Debug, Clone, Copy)]
pub enum Group {
    Calories,
    Custom,
    Keys,
    Exercises,
    ExerciseLogs,
    WorkoutPlans,
    WorkoutPlanLogs,
    Workouts,
    WorkoutLogs,
    Profiles,
    Protein,
    Sleep,
    Trainers,
    Announcements,
    Water,
    Users,
}

impl Display for Group {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let group = match self {
            Group::Calories => "calories",
            Group::Custom => "custom",
            Group::Keys => "api_keys",
            Group::Exercises => "exercises",
            Group::ExerciseLogs => "exercise_logs",
            Group::WorkoutPlans => "workout_plans",
            Group::WorkoutPlanLogs => "workout_plan_logs",
            Group::Workouts => "workouts",
            Group::WorkoutLogs => "workout_logs",
            Group::Profiles => "profiles",
            Group::Protein => "protein",
            Group::Sleep => "sleep",
            Group::Trainers => "trainers",
            Group::Announcements => "trainer_announcements",
            Group::Water => "water",
            Group::Users => "users",
        };
        write!(f, "{}", group)
    }
}

// -- Redis Functions
pub struct Cache;

impl Cache {
    // Get Value From Redis Cache
    pub async fn get<K: CacheKey>(
        pool: &State<Redis>,
        group: Group,
        key: K,
    ) -> Result<Value, Error> {
        tracing::info!("[Cache] ⚙️ Fetching Key From Redis Cache",);

        pool.get(format!("{}:{}", group, key.get()))
            .await
            .map_err(|error| {
                tracing::error!("[Redis]: {:?}", error);
                Error::Cache(error.to_string())
            })
    }

    // Sets A Value In Redis Cache
    pub async fn set<K: CacheKey>(
        pool: &Redis,
        group: Group,
        key: K,
        value: String,
    ) -> Result<(), Error> {
        tracing::info!("[Cache] ⚙️ Setting Key In Redis Cache");

        pool.set(
            format!("{}:{}", group, key.get()),
            value,
            Some(Expiration::EX(CACHE_EXPIRATION_TIME)),
            None,
            false,
        )
        .await
        .map_err(|error| {
            tracing::error!("[Redis]: {:?}", error);
            Error::Cache(error.to_string())
        })
    }

    // Deletes A Value From Redis Cache
    pub async fn delete<K: CacheKey>(
        pool: &State<Redis>,
        group: Group,
        key: K,
    ) -> Result<(), Error> {
        tracing::info!("[Cache] ⚙️ Deleting Key From Redis Cache",);

        pool.del(format!("{}:{}", group, key.get()))
            .await
            .map_err(|error| {
                tracing::error!("[Redis]: {:?}", error);
                Error::Cache(error.to_string())
            })
    }

    // Pings Redis For Health Check
    pub async fn ping(pool: &State<Redis>, message: Option<String>) -> Result<String, Error> {
        tracing::info!("[Cache] ⚙️ Pinging Redis For Health Check");

        pool.ping(message).await.map_err(|error| {
            tracing::error!("[Redis]: {:?}", error);
            Error::Cache(error.to_string())
        })
    }

    // Checks If A Key Exists In Redis Cache
    pub async fn exists<K: CacheKey>(pool: &Redis, group: Group, key: K) -> Result<bool, Error> {
        tracing::info!("[Cache] ⚙️ Checking Key Existence In Redis Cache",);

        pool.exists(format!("{}:{}", group, key.get()))
            .await
            .map_err(|error| {
                tracing::error!("[Redis]: {:?}", error);
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
