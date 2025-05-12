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
// 2. Clean Assets Directory Of Old Avatas
// 3. Cache All Exercises

use crate::{
    cache::redis::{Cache, Redis},
    constants,
    db::DB,
    models::{
        exercise::Exercise,
        keys::{APIKey, Status, UpdateAPIKey},
        user::User,
    },
};

// Verifies Expired API Keys
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
                    );

                    match User::find(key.user_id, connection).await {
                        Ok(user) => match APIKey::generate(&user, connection).await {
                            Ok(new_key) => tracing::info!(
                                "[+] ✅ New API Key Generated For {}\nUUID: {}",
                                user.username,
                                new_key.api_key
                            ),
                            Err(e) => tracing::error!("[+] ❌ Failed To Generate New Key: {}", e),
                        },
                        Err(e) => tracing::error!("[+] ❌ Failed To Find User: {}", e),
                    }
                }
                Err(e) => return Err(Box::new(e)),
            }
        }
    }

    tracing::info!("[+] ⏰ API Key Verification Completed");

    Ok(())
}

// Cleans Assets Directory Of Old Avatars
pub async fn clean_assets_directory() -> Result<(), Box<dyn std::error::Error>> {
    tracing::info!("[+] 🧹 Beginning Assets Directory Cleanup");

    // Get Assets Directory Path
    let assets_path = std::path::Path::new(constants::ASSETS_AVATARS_PATH);

    // Check If Assets Directory Exists
    if !assets_path.exists() {
        tracing::warn!("[+] ❌ Assets Directory Does Not Exist");
        return Ok(());
    }

    // Declare Retention Period
    // This Is The Period After Which Files Will Be Considered Old And Deleted
    let retention_period = chrono::Duration::days(constants::ASSETS_RETENTION_PERIOD);

    // Get Current Time
    // This Is Used To Compare Against The Last Modified Time Of Files
    let now = chrono::Utc::now();

    // Initialize Count of Deleted Files
    // This Will Keep Track Of How Many Files Have Been Deleted
    let mut count = 0;

    // Helper Function To Recursively Scan Directories
    fn scan_directory(
        dir_path: &std::path::Path,
        retention_period: chrono::Duration,
        now: chrono::DateTime<chrono::Utc>,
        count: &mut usize,
    ) -> std::io::Result<()> {
        for entry in std::fs::read_dir(dir_path)? {
            // Get Each Entry In The Directory
            let entry = entry?;

            // Get The Path Of The Entry
            let path = entry.path();

            // Check If The Entry Is A Directory Or A File
            if path.is_dir() {
                // Recursively Scan Subdirectories
                scan_directory(&path, retention_period, now, count)?;
            } else {
                // Get The Metadata Of The File
                let metadata = path.metadata()?;

                // Get The Last Modified Time Of The File
                let modified = metadata.modified()?;

                // Convert The Last Modified Time To A DateTime
                let modified_time = chrono::DateTime::<chrono::Utc>::from(modified);

                // Check If The File Is Older Than The Retention Period
                if now - modified_time > retention_period {
                    // Get The Relative Path Of The File (Readability)
                    let relative_path = path
                        .strip_prefix(dir_path.ancestors().nth(1).unwrap_or(dir_path))
                        .unwrap_or(&path)
                        .display();

                    tracing::info!("[+] 🗑️ Removing old avatar: {}", relative_path);

                    // Delete The File
                    std::fs::remove_file(&path)?;

                    // Increment The Count Of Deleted Files
                    *count += 1;
                }
            }
        }
        Ok(())
    }

    // Start The Recursive Scan From assets/avatars
    scan_directory(assets_path, retention_period, now, &mut count)?;

    tracing::info!(
        "[+] 🧹 Cleaned {} Old Avatar(s) From The Assets Directory",
        count
    );
    Ok(())
}

pub async fn cache_exercises(pool: &DB, redis: &Redis) -> Result<(), Box<dyn std::error::Error>> {
    tracing::info!("[+] 🗄️ Caching Exercises");

    let mut cached = 0;
    let mut skipped = 0;

    let group = "exercise";

    let connection = &mut pool.get().await?;

    let exercises = Exercise::all(connection).await?;

    if exercises.is_empty() {
        tracing::warn!("[+] 🗄️ No Exercises Found");
        return Ok(());
    }

    for exercise in exercises {
        let exists = Cache::exists(redis, group, exercise.exercise_id).await?;

        if exists {
            tracing::info!(
                "[+] 🗄️ Skipping Caching Exercise {}: {}",
                exercise.exercise_id,
                exercise.name
            );
            skipped += 1;
            continue;
        } else {
            tracing::info!(
                "[+] 🗄️ Caching Exercise {}: {}",
                exercise.exercise_id,
                exercise.name
            );

            Cache::set(
                redis,
                group,
                exercise.exercise_id,
                Cache::serialize(&exercise)?,
            )
            .await?;

            cached += 1;
        }
    }

    tracing::info!(
        "[+] 🗄️ Caching Completed. Set {} New Exercises, {} Already Cached.",
        cached,
        skipped
    );
    Ok(())
}
