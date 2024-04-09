use actix_web::web;
use actix_web::HttpResponse;
use actix_web::Responder;
use actix_web::Result;

use crate::routes::utils::reponses::ReturnError;
use crate::utils::get_body::get_body;

use crate::controller::users::structs::Create;
use crate::controller::users::structs::QueryParams;
use crate::controller::users::structs::Update;
use crate::controller::users::user_controller::UserController;

pub struct UsersRoute;

impl UsersRoute {
    pub async fn delete(user_id: web::Path<i32>) -> Result<impl Responder> {
        // body is loaded, now we can deserialize serde-json
        let user_id = user_id.into_inner();
        match UserController::delete(user_id) {
            Ok(res) => {
                return Ok(HttpResponse::Ok().json(res)); // if Successful, return the deleted data
            }
            Err(err) => {
                let not_found = err.to_string().to_lowercase().contains("not found");
                if not_found {
                    return Ok(HttpResponse::NotFound().json(ReturnError::<i32> {
                        error_msg: format!("User with id: {} not found", &user_id),
                        values: Some(user_id),
                    }));
                }
                return Ok(HttpResponse::BadRequest().json(ReturnError::<i32> {
                    error_msg: err.to_string(),
                    values: Some(user_id),
                }));
            }
        }
    }
    pub async fn create(payload: web::Payload) -> Result<impl Responder> {
        let new_user = match get_body::<Create>(payload).await {
            Ok(res) => res,
            Err(err) => return Ok(HttpResponse::BadRequest().json(err)),
        };

        match UserController::create(new_user) {
            Ok(res) => {
                return Ok(HttpResponse::Created().json(res));
            }
            Err(err) => {
                return Ok(HttpResponse::BadRequest().json(err));
            }
        }
    }
    pub async fn update(user_id: web::Path<i32>, payload: web::Payload) -> Result<impl Responder> {
        let user_id = user_id.into_inner();
        let mut new_user = match get_body::<Update>(payload).await {
            Ok(res) => res,
            Err(err) => return Ok(HttpResponse::BadRequest().json(err)),
        };

        new_user.updated_at = Some(chrono::Utc::now().naive_utc());
        match UserController::update(user_id, new_user) {
            Ok(res) => {
                return Ok(HttpResponse::Ok().json(res));
            }
            Err(err) => {
                let not_found = err.to_string().to_lowercase().contains("not found");
                let val = err.values.clone().unwrap();
                if not_found {
                    return Ok(HttpResponse::NotFound().json(ReturnError {
                        error_msg: format!("User with id: {} not found", user_id),
                        values: Some(val),
                    }));
                }
                return Ok(HttpResponse::BadRequest().json(ReturnError {
                    error_msg: err.to_string() + "Nada para o id:{user_id}",
                    values: Some(val),
                }));
            }
        }
    }
    pub async fn find_all(query_params: web::Query<QueryParams>) -> Result<impl Responder> {
        match UserController::find_all(query_params.into_inner()) {
            Ok(results) => return Ok(HttpResponse::Ok().json(results)),
            Err(err) => {
                return Ok(HttpResponse::BadRequest().json(err));
            }
        }
    }
    pub async fn find(user_id: web::Path<i32>) -> Result<impl Responder> {
        let user_id = user_id.into_inner();
        match UserController::find(user_id) {
            Ok(results) => return Ok(HttpResponse::Ok().json(results)),
            Err(err) => {
                return Ok(HttpResponse::NotFound().json(err));
            }
        }
    }
}
