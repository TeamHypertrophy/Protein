/*
______          _       _
| ___ \        | |     (_)
| |_/ / __ ___ | |_ ___ _ _ __
|  __/ '__/ _ \| __/ _ \ | '_ \
| |  | | | (_) | ||  __/ | | | |
\_|  |_|  \___/ \__\___|_|_| |_|

        Made with ❤️
*/

// use tokio-cron-scheduler to schedule jobs
// Current Jobs:
// 1. Check If An API Key Expired and Renew It (verify if quota => 0)

use tokio_cron_scheduler::{Job, JobScheduler};

use crate::{
    constants::API_QUOTA_LIMIT,
    db::DatabasePool,
    models::keys::{APIKey, UpdateAPIKey},
};

pub async fn verify_api_keys(pool: &DatabasePool) -> Result<(), Box<dyn std::error::Error>> {
    let connection = &mut pool.get().await?;

    // Get All API Keys
    let keys = APIKey::all(connection).await.unwrap();

    // Iterate Over Each Key
    for key in keys {
        println!("[*] Verifying API Key: {}", key.api_key);
    }

    Ok(())
}
