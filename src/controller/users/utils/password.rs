use std::env;

use crate::controller::users::structs::Create;
use dotenvy::dotenv;
pub struct PasswordUtils;

impl PasswordUtils {
    pub fn hash(mut user: Create) -> Create {
        dotenv().ok();

        let binding = env::var("SALT").expect("SALT must be set");
        let salt = binding.as_bytes();

        let password = user.password.as_bytes();
        let config = argon2::Config::default();

        let hash = argon2::hash_encoded(password, salt, &config).unwrap();
        user.password = hash; // Set new value for user password(Hashed one)

        return user;
    }
    pub fn verify(password: String, hash: String) -> bool {
        println!("{}",&password);
        match argon2::verify_encoded(&hash, password.as_bytes()) {
            Ok(_) => true,
            Err(err) => {
                println!("{:?}", err);
                return false;
            }
        }
    }
}
