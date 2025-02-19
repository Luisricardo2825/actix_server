use actix_web::web;
use actix_web::HttpResponse;
use actix_web::Responder;
use actix_web::Result;

use serde_json::Value;

use crate::models::db::connection::DbPool;
use crate::utils::get_body::get_body;

use crate::controller::custom::custom_controller::CustomController;
use crate::controller::QueryParams;

pub struct CustomRoute;

impl CustomRoute {
    pub async fn find_all(
        _pool: web::Data<DbPool>,
        table_name: web::Path<String>,
        query_params: web::Query<QueryParams>,
    ) -> Result<impl Responder> {
        match CustomController::find_all(table_name.into_inner(), query_params.into_inner()).await {
            Ok(results) => return Ok(HttpResponse::Ok().json(results)),
            Err(err) => {
                return Ok(HttpResponse::BadRequest().json(err));
            }
        }
    }

    pub async fn find_one(
        pool: web::Data<DbPool>,
        path: web::Path<(String, String)>,
        query_params: web::Query<QueryParams>,
    ) -> Result<impl Responder> {
        let (table_name, id) = path.into_inner();
        let pool = pool.into_inner();
        let controller = CustomController(pool);
        match controller
            .find_one(table_name, id, query_params.into_inner())
            .await
        {
            Ok(results) => return Ok(HttpResponse::Ok().json(results)),
            Err(err) => {
                return Ok(HttpResponse::BadRequest().json(err));
            }
        }
    }

    pub async fn create(
        path: web::Path<(String,)>,
        payload: web::Payload,
        query_params: web::Query<QueryParams>,
    ) -> Result<impl Responder> {
        let (table_name,) = path.into_inner();

        let table = match get_body::<Value>(payload).await {
            Ok(res) => res,
            Err(err) => return Ok(HttpResponse::BadRequest().json(err)),
        };

        match CustomController::create(table_name, table, query_params.into_inner()).await {
            Ok(res) => {
                return Ok(HttpResponse::Created().json(res));
            }
            Err(err) => {
                return Ok(HttpResponse::BadRequest().json(err));
            }
        }
    }

    // pub async fn delete(payload: web::Payload) -> Result<impl Responder> {
    //     let table = match get_body::<Delete>(payload).await {
    //         Ok(res) => res,
    //         Err(err) => return Ok(HttpResponse::BadRequest().json(err)),
    //     };

    //     match CustomController::delete(table.id) {
    //         Ok(res) => {
    //             return Ok(HttpResponse::Ok().json(res)); // if Successful, return the deleted data
    //         }
    //         Err(err) => {
    //             let not_found = err.to_string().to_lowercase().contains("not found");
    //             if not_found {
    //                 return Ok(HttpResponse::NotFound().json(ReturnError {
    //                     error_msg: format!("table with id: {} not found", &table.id),
    //                     values: Some(serde_json::to_value(table).unwrap()),
    //                 }));
    //             }
    //             return Ok(HttpResponse::BadRequest().json(ReturnError {
    //                 error_msg: err.to_string(),
    //                 values: Some(serde_json::to_value(table).unwrap()),
    //             }));
    //         }
    //     }
    // }
    // pub async fn create(payload: web::Payload) -> Result<impl Responder> {
    //     let table = match get_body::<CreateTableRequest>(payload).await {
    //         Ok(res) => res,
    //         Err(err) => return Ok(HttpResponse::BadRequest().json(err)),
    //     };

    //     match CustomController::create(table) {
    //         Ok(res) => {
    //             return Ok(HttpResponse::Created().json(res));
    //         }
    //         Err(err) => {
    //             return Ok(HttpResponse::BadRequest().json(err));
    //         }
    //     }
    // }
    // pub async fn update(post_id: web::Path<i32>, payload: web::Payload) -> Result<impl Responder> {
    //     let post_id = post_id.into_inner();
    //     let mut table = match get_body::<Update>(payload).await {
    //         Ok(res) => res,
    //         Err(err) => return Ok(HttpResponse::BadRequest().json(err)),
    //     };

    //     table.updated_at = Some(chrono::Utc::now().naive_utc()); // update the updated_at field with the current time

    //     match CustomController::update(post_id, table) {
    //         Ok(res) => {
    //             return Ok(HttpResponse::Ok().json(res));
    //         }
    //         Err(err) => {
    //             let not_found = err.to_string().to_lowercase().contains("not found");
    //             let val = err.values.clone().unwrap();
    //             if not_found {
    //                 return Ok(HttpResponse::NotFound().json(ReturnError {
    //                     error_msg: format!("post with id: {} not found", &post_id),
    //                     values: Some(val),
    //                 }));
    //             }
    //             return Ok(HttpResponse::BadRequest().json(ReturnError {
    //                 error_msg: err.to_string(),
    //                 values: Some(val),
    //             }));
    //         }
    //     }
    // }
    // pub async fn find(post_id: web::Path<i32>) -> Result<impl Responder> {
    //     match CustomController::find(post_id.into_inner()) {
    //         Ok(results) => return Ok(HttpResponse::Ok().json(results)),
    //         Err(err) => {
    //             return Ok(HttpResponse::NotFound().json(err));
    //         }
    //     }
    // }
    // pub async fn find_table_by_name(table_id: web::Path<String>) -> Result<impl Responder> {
    //     match CustomController::find_by_name(table_id.into_inner()) {
    //         Ok(results) => return Ok(HttpResponse::Ok().json(results)),
    //         Err(err) => {
    //             return Ok(HttpResponse::NotFound().json(err));
    //         }
    //     }
    // }
}
