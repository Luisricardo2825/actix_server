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
}
