use argon2::Config;
use rand::Rng;

pub fn generate_password(password: String) -> String {
    let config = Config::default();
    let salt: [u8; 32] = rand::thread_rng().gen();

    let hash = argon2::hash_encoded(password.as_bytes(), &salt, &config).unwrap();

    return hash;
}
