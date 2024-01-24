// /src/utils/hash_and_salt.rs

// Used the docs implementation.
// https://docs.rs/argon2/latest/argon2/

// Hashing and Salting

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};

// In order to not connect to the DB
// in two seperate files, we use this
// publicly...
pub fn generate_salt() -> SaltString {
    SaltString::generate(&mut OsRng)
}

// Only the hashed password is stored in the database.
pub fn hash_password(
    password: &[u8],
    salt: &SaltString,
) -> Result<String, argon2::password_hash::Error> {
    let password_hash = Argon2::default().hash_password(password, salt)?;
    Ok(password_hash.to_string())
}

// nevermind clearly idk how hashes work.
/*
pub fn extract_hash(hashed_password: &str) -> std::string::String {
    let parts: Vec<&str> = hashed_password.split('$').collect();
    // last part
    parts[parts.len() - 1].to_string()
}
*/
