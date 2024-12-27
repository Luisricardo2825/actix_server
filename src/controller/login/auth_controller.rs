use std::env;

use crate::{
    controller::users::{user_controller::UserController, utils::password::PasswordUtils},
    models::users_model::User,
    routes::utils::reponses::ReturnError,
};
use dotenvy::dotenv;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

pub struct AuthController;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoginData {
    email: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,
    pub api_rights: bool,
    pub admin_rights: bool,
}

impl AuthController {
    pub async fn login(login_data: LoginData) -> Result<User, ReturnError> {
        let err_default =
            ReturnError::new("Invalid email or password".to_string(), login_data.clone());

        let user_email = login_data.email.clone();
        let user = match UserController::find_by_email(user_email) {
            Ok(res) => res,
            Err(_) => return Err(err_default),
        };

        let valid_pass = PasswordUtils::verify(&login_data.password, &user.password);

        if !valid_pass {
            return Err(err_default);
        }

        Ok(user)
    }

    pub fn verify_jwt(token: String) -> (bool, Option<Claims>) {
        dotenv().ok();
        let secret = env::var("SECRET").expect("SALT must be set");

        // my_claims is a struct that implements Serialize
        // This will create a JWT using HS256 as algorithm
        let token = decode::<Claims>(
            &token,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::new(Algorithm::HS256),
        );

        match token {
            Ok(token) => (true, Some(token.claims)),
            Err(_) => (false, None),
        }
    }
}
