// /src/utils/utils.rs

// Used the docs implementation.
// https://docs.rs/argon2/latest/argon2/

use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};

// In order to not connect to the DB
// in two seperate files, we use this
// publicly...
pub fn generate_salt() -> SaltString {
    let salt = SaltString::generate(&mut OsRng);
    salt
}

// Only the hashed password is stored in the database.
pub fn hash_password(
    password: &[u8],
    salt: &SaltString
) -> Result<String, argon2::password_hash::Error> {
    let password_hash = Argon2::default().hash_password(password, salt)?;
    Ok(password_hash.to_string())
}
