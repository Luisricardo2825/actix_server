use actix_web::web;
use actix_web::HttpResponse;
use actix_web::Responder;
use actix_web::Result;

use crate::controller::tables::structs::CreateTableRequest;
use crate::controller::tables::table_controller::TableController;
use crate::controller::Controller;
use crate::routes::utils::reponses::ReturnError;
use crate::utils::get_body::get_body;

use crate::controller::tables::structs::Delete;
use crate::controller::tables::structs::QueryParams;
use crate::controller::tables::structs::Update;

pub struct TableRoute;

impl TableRoute {
    pub async fn delete(payload: web::Payload) -> Result<impl Responder> {
        let table = match get_body::<Delete>(payload).await {
            Ok(res) => res,
            Err(err) => return Ok(HttpResponse::BadRequest().json(err)),
        };

        match TableController::delete(table.id) {
            Ok(res) => {
                return Ok(HttpResponse::Ok().json(res)); // if Successful, return the deleted data
            }
            Err(err) => {
                let not_found = err.to_string().to_lowercase().contains("not found");
                if not_found {
                    return Ok(HttpResponse::NotFound().json(ReturnError {
                        error_msg: format!("table with id: {} not found", &table.id),
                        values: Some(serde_json::to_value(table).unwrap()),
                    }));
                }
                return Ok(HttpResponse::BadRequest().json(ReturnError {
                    error_msg: err.to_string(),
                    values: Some(serde_json::to_value(table).unwrap()),
                }));
            }
        }
    }
    pub async fn create(payload: web::Payload) -> Result<impl Responder> {
        let table = match get_body::<CreateTableRequest>(payload).await {
            Ok(res) => res,
            Err(err) => return Ok(HttpResponse::BadRequest().json(err)),
        };

        match TableController::create(table) {
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
        let mut table = match get_body::<Update>(payload).await {
            Ok(res) => res,
            Err(err) => return Ok(HttpResponse::BadRequest().json(err)),
        };

        table.updated_at = Some(chrono::Utc::now().naive_utc()); // update the updated_at field with the current time

        match TableController::update(post_id, table) {
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
        match TableController::find_all(query_params.into_inner()) {
            Ok(results) => return Ok(HttpResponse::Ok().json(results)),
            Err(err) => {
                return Ok(HttpResponse::BadRequest().json(err));
            }
        }
    }
    pub async fn find(post_id: web::Path<i32>) -> Result<impl Responder> {
        match TableController::find(post_id.into_inner()) {
            Ok(results) => return Ok(HttpResponse::Ok().json(results)),
            Err(err) => {
                return Ok(HttpResponse::NotFound().json(err));
            }
        }
    }
    pub async fn find_table_by_name(table_id: web::Path<String>) -> Result<impl Responder> {
        match TableController::find_by_name(table_id.into_inner()) {
            Ok(results) => return Ok(HttpResponse::Ok().json(results)),
            Err(err) => {
                return Ok(HttpResponse::NotFound().json(err));
            }
        }
    }
}
