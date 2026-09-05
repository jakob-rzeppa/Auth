use argon2::{Argon2, password_hash::PasswordHasher};

use crate::config::CONFIG;

pub enum HashPasswordError {
    HashingError,
}

pub fn hash_password(password: &str) -> Result<String, HashPasswordError> {
    let argon2 = Argon2::new_with_secret(
        CONFIG.database_pepper().as_bytes(),
        argon2::Algorithm::default(),
        argon2::Version::default(),
        argon2::Params::default(),
    )
    .map_err(|e| {
        eprintln!("Initializing Argon2 failed: {}", e.to_string());
        HashPasswordError::HashingError
    })?;

    argon2
        .hash_password(password.as_bytes())
        .map(|hash| hash.to_string())
        .map_err(|e| {
            eprintln!("Hashing a password failed: {}", e.to_string());
            HashPasswordError::HashingError
        })
}
