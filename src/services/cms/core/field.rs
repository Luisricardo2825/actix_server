use crate::controller::fields::field_controller::FieldController;
use crate::controller::fields::structs::CreateField;
use crate::controller::fields::structs::UpdateField;
use crate::controller::QueryParams;
use crate::routes::utils::reponses::ReturnError;
use crate::utils::get_body::get_body;
use actix_web::web;
use actix_web::web::Payload;
use actix_web::HttpResponse;
use actix_web::Responder;
use actix_web::Result;

pub struct FieldRoute;

impl FieldRoute {
    // Fields routes
    pub async fn create(path: web::Path<(String,)>, payload: Payload) -> Result<impl Responder> {
        let (table_name,) = path.into_inner();
        use std::result::Result::Ok;
        match get_body::<Vec<CreateField>>(payload).await {
            Ok(tables) => match FieldController::create_fields(table_name, tables) {
                Ok(res) => {
                    return Ok(HttpResponse::Created().json(res));
                }
                Err(err) => {
                    return Ok(HttpResponse::BadRequest().json(err));
                }
            },
            Err(err) => return Ok(HttpResponse::BadRequest().json(err)),
        }
    }

    pub async fn find(path: web::Path<(String, String)>) -> Result<impl Responder> {
        let (table_name, field) = path.into_inner();

        // Try convert "field" from String to i32

        match field.parse::<i32>() {
            Ok(field_id) => {
                match FieldController::find(field_id) {
                    Ok(results) => return Ok(HttpResponse::Ok().json(results)),
                    Err(err) => {
                        return Ok(HttpResponse::NotFound().json(err));
                    }
                }
            }
            Err(_) => match FieldController::find_field_by_name(table_name, field) {
                Ok(results) => return Ok(HttpResponse::Ok().json(results)),
                Err(err) => {
                    return Ok(HttpResponse::NotFound().json(err));
                }
            },
        }
    }
    pub async fn find_all(
        path: web::Path<(String,)>,
        query_params: web::Query<QueryParams>,
    ) -> Result<impl Responder> {
        let (table_name,) = path.into_inner();
        let query_params = query_params.into_inner();

        match FieldController::find_all_fields(table_name, query_params) {
            Ok(results) => return Ok(HttpResponse::Ok().json(results)),
            Err(err) => {
                return Ok(HttpResponse::NotFound().json(err));
            }
        }
    }

    pub async fn delete_by_name(path: web::Path<(String, String)>) -> Result<impl Responder> {
        let (table_name, field_name) = path.into_inner();

        let result = match FieldController::delete_field_by_name(&table_name, &field_name) {
            Ok(res) => {
                HttpResponse::Ok().json(res) // if Successful, return the deleted data
            }
            Err(err) => {
                let not_found = err.to_string().to_lowercase().contains("not found");
                if not_found {
                    HttpResponse::NotFound().json(ReturnError {
                        error_msg: format!("table with name: {} not found", &table_name),
                        values: None,
                    })
                } else {
                    HttpResponse::BadRequest().json(ReturnError {
                        error_msg: err.to_string(),
                        values: None,
                    })
                }
            }
        };

        Ok(result)
    }
    pub async fn update(
        field_id: web::Path<(String, i32)>,
        payload: web::Payload,
    ) -> Result<impl Responder> {
        let (_, field_id) = field_id.into_inner();
        let field = match get_body::<UpdateField>(payload).await {
            Ok(res) => res,
            Err(err) => return Ok(HttpResponse::BadRequest().json(err)),
        };

        match FieldController::update_field(field_id, field) {
            Ok(res) => {
                return Ok(HttpResponse::Ok().json(res));
            }
            Err(err) => {
                let not_found = err.to_string().to_lowercase().contains("not found");
                let val = err.values.clone().unwrap();
                if not_found {
                    return Ok(HttpResponse::NotFound().json(ReturnError {
                        error_msg: format!("Field with id: {} not found", &field_id),
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
}
