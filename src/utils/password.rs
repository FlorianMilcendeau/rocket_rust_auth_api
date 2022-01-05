use argon2::{Config, Error};
use rand::Rng;

pub fn generate_password(password: String) -> String {
    let config = Config::default();
    let salt: [u8; 32] = rand::thread_rng().gen();

    let hash = argon2::hash_encoded(password.as_bytes(), &salt, &config).unwrap();

    return hash;
}

pub fn verify_password(password_hash: &str, password: &[u8]) -> Result<bool, Error> {
    argon2::verify_encoded(password_hash, password)
}
