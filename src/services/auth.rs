use std::env;

use actix_web::{web, HttpResponse, Responder, Result};
use dotenvy::dotenv;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

use crate::{
    controller::login::auth_controller::{AuthController, Claims, LoginData},
    models::users_model::User,
    utils::get_body::get_body,
};

#[derive(Serialize, Deserialize)]

struct TokenReturn {
    token: String,
}
pub struct AuthService;

impl AuthService {
    pub async fn login(payload: web::Payload) -> Result<impl Responder> {
        let login_data = match get_body::<LoginData>(payload).await {
            Ok(res) => res,
            Err(err) => return Ok(HttpResponse::BadRequest().json(err)),
        };

        let user = AuthController::login(login_data).await;
        let user = match user {
            Ok(user) => user,
            Err(err) => return Ok(HttpResponse::NotFound().json(err)),
        };

        let res = HttpResponse::Accepted().json(TokenReturn {
            token: generate_jwt(user),
        });

        Ok(res)
    }
}

fn generate_jwt(user_data: User) -> String {
    dotenv().ok();
    let secret = env::var("SECRET").expect("SALT must be set");

    let now = chrono::Utc::now() + chrono::Duration::days(1);

    let my_claims = Claims {
        exp: now.timestamp() as usize,
        api_rights: user_data.api_rights,
        admin_rights: user_data.admin,
    };

    let token = encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .unwrap();

    token
}
