use std::env;

use crate::{
    controller::users::{user_controller::UserController, utils::password::PasswordUtils},
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
struct Claims {
    login_data: LoginData,
    exp: usize,
}

impl AuthController {
    pub async fn login(login_data: LoginData) -> Result<LoginData, ReturnError<LoginData>> {
        let err_default = ReturnError::<LoginData> {
            error_msg: "Invalid user or password".to_string(),
            values: Some(login_data.clone()),
        };

        let user_email = login_data.email.clone();
        let user = match UserController::find_by_email(user_email) {
            Ok(res) => res,
            Err(_) => return Err(err_default),
        };

        let valid_pass = PasswordUtils::verify(login_data.password.clone(), user.password);

        if !valid_pass {
            return Err(err_default);
        }

        Ok(login_data)
    }

    pub fn verify_jwt(token: String) -> bool {
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
            Ok(_) => true,
            Err(_) => false,
        }
    }
}
