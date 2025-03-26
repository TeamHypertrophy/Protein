/*
______          _       _
| ___ \        | |     (_)
| |_/ / __ ___ | |_ ___ _ _ __
|  __/ '__/ _ \| __/ _ \ | '_ \
| |  | | | (_) | ||  __/ | | | |
\_|  |_|  \___/ \__\___|_|_| |_|

        Made with ❤️
*/
use std::sync::{Arc, LazyLock};

pub type Config = Arc<Admin>;

// Struct Containing Admin Information
pub struct Admin {
    // Static Vector of Admin Routes
    pub routes: LazyLock<Vec<&'static str>>,
    // API Master Key
    pub master_key: String,
    // Application Environment (Development, Production, etc.)
    pub app_env: String,
    // Host URL For API
    pub host_url: String,
    // Host URL For User Avatars
    pub avatar_host_url: String,
    // Password Salt Used For Argon-2 Hashes
    pub password_salt: String,
    // SMTP Username Config
    pub smtp_user: String,
    // STMP User Config
    pub smtp_username: String,
}
