/*
______          _       _
| ___ \        | |     (_)
| |_/ / __ ___ | |_ ___ _ _ __
|  __/ '__/ _ \| __/ _ \ | '_ \
| |  | | | (_) | ||  __/ | | | |
\_|  |_|  \___/ \__\___|_|_| |_|

        Made with ❤️
*/

use rand::{Rng, prelude::ThreadRng};
use argon2::{self, Config};

use crate::errors::Error;

// Generate A Hashed Password Using Argon2
// https://github.com/sru-systems/rust-argon2
pub fn generate(salt: &String, password: String) -> Result<String, Error> {
    // Create Argon2 Config
    let config: Config<'_> = Config::default();

    // Hash Password
    argon2::hash_encoded(password.as_bytes(), salt.as_bytes(), &config).map_err(|error| {
        tracing::error!("[!] Password Hashing Error {:?}", error);
        Error::Internal(error.to_string())
    })
}

// Verify A Password Against A Hashed Password
pub fn verify(hashed_password: String, password: String) -> Result<bool, Error> {
    // Verify Argon2 Password Hash
    argon2::verify_encoded(hashed_password.as_str(), password.as_bytes()).map_err(|error| {
        tracing::error!("[!] Password Verification Error: {:?}", error);
        Error::Internal(error.to_string())
    })
}

// Generate A Random 6-Digit Verification Code For Multi-Factor Authentication
pub fn random() -> String {
    let mut rng: ThreadRng = rand::rng();

    rng.random_range(100000..=999999).to_string()
}
