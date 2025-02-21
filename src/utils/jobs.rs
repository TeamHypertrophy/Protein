/*
______          _       _
| ___ \        | |     (_)
| |_/ / __ ___ | |_ ___ _ _ __
|  __/ '__/ _ \| __/ _ \ | '_ \
| |  | | | (_) | ||  __/ | | | |
\_|  |_|  \___/ \__\___|_|_| |_|

        Made with ❤️
*/

// Current Jobs:
// 1. Check If An API Key Expired and Renew It (verify if quota => 0)

use crate::{db::DatabasePool, models::keys::APIKey};

pub async fn verify_api_keys(pool: &DatabasePool) -> Result<(), Box<dyn std::error::Error>> {
    tracing::info!("[+] ⏰ Beginning API Key Verification Job");
    // Get Database Connection
    let connection = &mut pool.get().await?;

    // Get All API Keys
    let keys = match APIKey::all(connection).await {
        Ok(keys) => keys,
        Err(e) => return Err(Box::new(e)),
    };

    // TODO: Implement API Key Verification
    tracing::info!("[+] ⏰ API Key Verification Completed");
    Ok(())
}
