use std::env;

use actix_web::{web, HttpResponse, Responder, Result};
use dotenvy::dotenv;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

use crate::{controller::login::auth_controller::LoginData, utils::get_body::get_body};

pub struct AuthService;

impl AuthService {
    pub async fn login(payload: web::Payload) -> Result<impl Responder> {
        let login_data = match get_body::<LoginData>(payload).await {
            Ok(res) => res,
            Err(err) => return Ok(HttpResponse::BadRequest().json(err)),
        };

        let res = HttpResponse::Accepted().body(generate_jwt(login_data));

        Ok(res)
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

    let now = chrono::Utc::now() + chrono::Duration::days(1);

    let my_claims = Claims {
        login_data: user_data,
        exp: now.timestamp() as usize,
    };

    let token = encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .unwrap();

    token
}
