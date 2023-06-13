use actix_web::web;
use actix_web::HttpResponse;
use actix_web::Responder;
use actix_web::Result;

use crate::routes::utils::reponses::ReturnError;
use crate::utils::get_body::get_body;

use crate::controller::users::structs::Create;
use crate::controller::users::structs::Delete;
use crate::controller::users::structs::QueryParams;
use crate::controller::users::structs::Update;
use crate::controller::users::user_controller::UserController;

pub struct UsersRoute;

impl UsersRoute {
    pub async fn delete(payload: web::Payload) -> Result<impl Responder> {
        // body is loaded, now we can deserialize serde-json
        let new_user = match get_body::<Delete>(payload).await {
            Ok(res) => res,
            Err(err) => return Ok(HttpResponse::BadRequest().json(err)),
        };

        match UserController::delete(new_user.id) {
            Ok(res) => {
                return Ok(HttpResponse::Ok().json(res)); // if Successful, return the deleted data
            }
            Err(err) => {
                let not_found = err.to_string().to_lowercase().contains("not found");
                if not_found {
                    return Ok(HttpResponse::NotFound().json(ReturnError::<Delete> {
                        error_msg: format!("User with id: {} not found", &new_user.id),
                        values: Some(new_user),
                    }));
                }
                return Ok(HttpResponse::BadRequest().json(ReturnError::<Delete> {
                    error_msg: err.to_string(),
                    values: Some(new_user),
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
    pub async fn update(payload: web::Payload) -> Result<impl Responder> {
        let new_user = match get_body::<Update>(payload).await {
            Ok(res) => res,
            Err(err) => return Ok(HttpResponse::BadRequest().json(err)),
        };

        match UserController::update(new_user) {
            Ok(res) => {
                return Ok(HttpResponse::Ok().json(res));
            }
            Err(err) => {
                let not_found = err.to_string().to_lowercase().contains("not found");
                let val = err.values.clone().unwrap();
                if not_found {
                    return Ok(HttpResponse::NotFound().json(ReturnError {
                        error_msg: format!("User with id: {} not found", &val.id),
                        values: Some(val),
                    }));
                }
                return Ok(HttpResponse::BadRequest().json(ReturnError {
                    error_msg: err.to_string(),
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
        match UserController::find(user_id.into_inner()) {
            Ok(results) => return Ok(HttpResponse::Ok().json(results)),
            Err(err) => {
                return Ok(HttpResponse::NotFound().json(err));
            }
        }
    }
}
