// /src/utils/utils.rs

use rand::Rng;
use argon2::{ self, Config, ThreadMode, Variant, Version };

fn generate_salt() -> String {
    // MAKE SURE TO UPDATE THE WHOLE DATABASE
    // IN THE CASE OF USING A DIFFERENT SIZE
    // THIS SHOULDN'T BE CHANGED ON A EXISTING DATABASE!!
    const SALT_LENGTH: usize = 32;

    let mut rng = rand::thread_rng();
    let salt: String = (0..SALT_LENGTH)
        .map(|_| rng.sample(rand::distributions::Alphanumeric))
        .collect();

    salt

}

// Only the hashed password is stored in the database.
pub fn hash_password(password: &str) -> String {
    let config = Config {
        variant: Variant::Argon2id,
        version: Version::Version13,
        threads: ThreadMode::Parallel,
        ..Default::default()
    };

    let generated_salt: String = generate_salt();

    let hash = argon2::hash_encoded(password.as_bytes(), generated_salt, &config).unwrap();

    hash
}
