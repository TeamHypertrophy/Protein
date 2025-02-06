/*
______          _       _
| ___ \        | |     (_)
| |_/ / __ ___ | |_ ___ _ _ __
|  __/ '__/ _ \| __/ _ \ | '_ \
| |  | | | (_) | ||  __/ | | | |
\_|  |_|  \___/ \__\___|_|_| |_|

        Made with ❤️
*/
use std::env;

use argon2::{self, Config};
use dotenvy::dotenv;

use crate::responders::ProteinError;

pub fn generate_hashed_password(password: String) -> Result<String, ProteinError> {
    // Load Environment Variables
    dotenv().ok();

    // Get Password Salt
    let salt: String =
        env::var("PASSWORD_SALT").expect("[!] PASSWORD_SALT Environment Variable Must Be Set");

    // Create Argon2 Config
    let config = Config::default();

    // Hash Password
    argon2::hash_encoded(password.as_bytes(), salt.as_bytes(), &config).map_err(|error| {
        tracing::error!("[!] Password Hashing Error {:?}", error);
        ProteinError::Internal(error.to_string())
    })
}

pub fn verify_password(hashed_password: String, password: String) -> Result<bool, ProteinError> {
    // Verify Argon2 Password Hash
    argon2::verify_encoded(hashed_password.as_str(), password.as_bytes()).map_err(|error| {
        tracing::error!("[!] Password Verification Error: {:?}", error);
        ProteinError::Internal(error.to_string())
    })
}
