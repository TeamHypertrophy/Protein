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
// 1. Check If An API Key Expired

use crate::{
    db::DB,
    models::keys::{APIKey, Status, UpdateAPIKey},
};

pub async fn verify_api_keys(pool: &DB) -> Result<(), Box<dyn std::error::Error>> {
    tracing::info!("[+] ⏰ Beginning API Key Verification Job");

    // Get Database Connection
    let connection = &mut pool.get().await?;

    // Get All API Keys
    let keys: Vec<APIKey> = match APIKey::all(connection).await {
        Ok(keys) => keys,
        Err(e) => return Err(Box::new(e)),
    };

    // Get Current Time
    let now: i64 = chrono::Utc::now().naive_utc().and_utc().timestamp();

    // Iterate Over API Keys
    for key in keys {
        let expired: i64 = key.expires_at.and_utc().timestamp();

        // Check If Key Expired
        if now >= expired {
            tracing::info!(
                "[+] 🔔 API Key: {} Expired\nUUID: {}",
                key.key_id,
                key.api_key
            );

            // Update Key
            let data = UpdateAPIKey {
                expires_at: Some(key.expires_at),
                role: Some(key.role),
                revoked_reason: Some("Expired".to_string()),
                status: Some(Status::Expired),
                quota: Some(0),
            };

            match APIKey::update(key.api_key, data, connection).await {
                Ok(_) => {
                    tracing::info!(
                        "[+] ✅ API Key: {} Updated\nUUID: {}",
                        key.key_id,
                        key.api_key
                    )
                }
                Err(e) => return Err(Box::new(e)),
            }
        }
    }

    tracing::info!("[+] ⏰ API Key Verification Completed");

    Ok(())
}
