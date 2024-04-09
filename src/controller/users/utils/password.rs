use std::env;

use dotenvy::dotenv;
pub struct PasswordUtils;

impl PasswordUtils {
    pub fn hash(password: String) -> String {
        dotenv().ok();

        let binding = env::var("SALT").expect("SALT must be set");
        let salt = binding.as_bytes();

        let password = password.as_bytes();
        let config = argon2::Config::default();

        let hash = argon2::hash_encoded(password, salt, &config).unwrap();

        return hash;
    }
    pub fn verify(password: &str, hash: &str) -> bool {
        argon2::verify_encoded(hash, password.as_bytes()).unwrap()
    }
}