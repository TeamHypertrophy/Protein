/*
______          _       _
| ___ \        | |     (_)
| |_/ / __ ___ | |_ ___ _ _ __
|  __/ '__/ _ \| __/ _ \ | '_ \
| |  | | | (_) | ||  __/ | | | |
\_|  |_|  \___/ \__\___|_|_| |_|

        Made with ❤️
*/

use rand::Rng;
use argon2::{self, Config};

use crate::errors::Error;

pub fn generate(password: String) -> Result<String, Error> {
    // Get Password Salt
    let salt: String =
        std::env::var("PASSWORD_SALT").expect("[!] PASSWORD_SALT Environment Variable Must Be Set");

    // Create Argon2 Config
    let config = Config::default();

    // Hash Password
    argon2::hash_encoded(password.as_bytes(), salt.as_bytes(), &config).map_err(|error| {
        tracing::error!("[!] Password Hashing Error {:?}", error);
        Error::Internal(error.to_string())
    })
}

pub fn verify(hashed_password: String, password: String) -> Result<bool, Error> {
    // Verify Argon2 Password Hash
    argon2::verify_encoded(hashed_password.as_str(), password.as_bytes()).map_err(|error| {
        tracing::error!("[!] Password Verification Error: {:?}", error);
        Error::Internal(error.to_string())
    })
}

pub fn random() -> String {
    let mut rng = rand::rng();

    rng.random_range(100000..=999999).to_string()
}
