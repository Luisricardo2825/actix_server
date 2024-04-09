use actix_web::web;
use actix_web::HttpResponse;
use actix_web::Responder;
use actix_web::Result;

use crate::routes::utils::reponses::ReturnError;
use crate::utils::get_body::get_body;

use crate::controller::posts::post_controller::PostController;
use crate::controller::posts::structs::Create;
use crate::controller::posts::structs::Delete;
use crate::controller::posts::structs::QueryParams;
use crate::controller::posts::structs::Update;

pub struct PostsRoute;

impl PostsRoute {
    pub async fn delete(payload: web::Payload) -> Result<impl Responder> {
        let new_post = match get_body::<Delete>(payload).await {
            Ok(res) => res,
            Err(err) => return Ok(HttpResponse::BadRequest().json(err)),
        };

        match PostController::delete(new_post.id) {
            Ok(res) => {
                return Ok(HttpResponse::Ok().json(res)); // if Successful, return the deleted data
            }
            Err(err) => {
                let not_found = err.to_string().to_lowercase().contains("not found");
                if not_found {
                    return Ok(HttpResponse::NotFound().json(ReturnError::<Delete> {
                        error_msg: format!("post with id: {} not found", &new_post.id),
                        values: Some(new_post),
                    }));
                }
                return Ok(HttpResponse::BadRequest().json(ReturnError::<Delete> {
                    error_msg: err.to_string(),
                    values: Some(new_post),
                }));
            }
        }
    }
    pub async fn create(payload: web::Payload) -> Result<impl Responder> {
        let new_post = match get_body::<Create>(payload).await {
            Ok(res) => res,
            Err(err) => return Ok(HttpResponse::BadRequest().json(err)),
        };

        match PostController::create(new_post) {
            Ok(res) => {
                return Ok(HttpResponse::Created().json(res));
            }
            Err(err) => {
                return Ok(HttpResponse::BadRequest().json(err));
            }
        }
    }
    pub async fn update(post_id: web::Path<i32>, payload: web::Payload) -> Result<impl Responder> {
        let post_id = post_id.into_inner();
        let mut new_post = match get_body::<Update>(payload).await {
            Ok(res) => res,
            Err(err) => return Ok(HttpResponse::BadRequest().json(err)),
        };

        new_post.updated_at = Some(chrono::Utc::now().naive_utc()); // update the updated_at field with the current time

        match PostController::update(post_id, new_post) {
            Ok(res) => {
                return Ok(HttpResponse::Ok().json(res));
            }
            Err(err) => {
                let not_found = err.to_string().to_lowercase().contains("not found");
                let val = err.values.clone().unwrap();
                if not_found {
                    return Ok(HttpResponse::NotFound().json(ReturnError {
                        error_msg: format!("post with id: {} not found", &post_id),
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
        match PostController::find_all(query_params.into_inner()) {
            Ok(results) => return Ok(HttpResponse::Ok().json(results)),
            Err(err) => {
                return Ok(HttpResponse::BadRequest().json(err));
            }
        }
    }
    pub async fn find(post_id: web::Path<i32>) -> Result<impl Responder> {
        match PostController::find(post_id.into_inner()) {
            Ok(results) => return Ok(HttpResponse::Ok().json(results)),
            Err(err) => {
                return Ok(HttpResponse::NotFound().json(err));
            }
        }
    }
}
