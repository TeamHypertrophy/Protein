// Fred
use fred::prelude::*;

// .env Loader
use dotenvy::dotenv;
use std::env;

// STD
use std::time::Duration;

pub type RedisPool = Pool;

pub async fn build_redis() -> Result<Pool, Error> {
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
