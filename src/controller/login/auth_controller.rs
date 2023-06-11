use std::env;

use crate::{
    controller::users::{user_controller::UserController, utils::password::PasswordUtils},
    routes::utils::reponses::ReturnError,
    utils::deserialize_payload::deserialize_payload,
};
use actix_web::{web, HttpResponse, Responder, Result};
use chrono::Duration;
use dotenvy::dotenv;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

pub struct AuthController;

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginData {
    email: String,
    password: String,
}

impl AuthController {
    pub async fn login(payload: web::Payload) -> Result<impl Responder> {
        let err_default = ReturnError::<i32> {
            error_msg: "Invalid user or password".to_string(),
            values: None,
        };
        let mut json = web::BytesMut::new();

        json = match deserialize_payload(json, payload).await {
            Ok(res) => res,
            Err(err) => {
                return Ok(HttpResponse::BadRequest().json(err));
            }
        };

        let login_data = match serde_json::from_slice::<LoginData>(&json) {
            Ok(res) => res,
            Err(err) => {
                return Ok(HttpResponse::BadRequest().json(ReturnError::<String> {
                    error_msg: format!("Invalid JSON: {}", err.to_string()),
                    values: None,
                }));
            }
        };

        let user_email = login_data.email.clone();
        let user = match UserController::find_by_email(user_email).await {
            Ok(res) => res,
            Err(_) => return Ok(HttpResponse::BadRequest().json(err_default)),
        };

        let valid_pass = PasswordUtils::verify(login_data.password.clone(), user.password);

        if !valid_pass {
            return Ok(HttpResponse::Unauthorized().json(ReturnError::<LoginData> {
                error_msg: "Invalid user or password".to_string(),
                values: Some(login_data),
            }));
        }

        let res = HttpResponse::Accepted().body(generate_jwt(login_data));

        Ok(res)
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

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    login_data: LoginData,
    exp: usize,
}
fn generate_jwt(user_data: LoginData) -> String {
    dotenv().ok();
    let secret = env::var("SECRET").expect("SALT must be set");

    let now = chrono::Utc::now() + Duration::days(1);

    let my_claims = Claims {
        login_data: user_data,
        exp: now.timestamp() as usize,
    };

    // my_claims is a struct that implements Serialize
    // This will create a JWT using HS256 as algorithm
    let token = encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .unwrap();

    token
}
