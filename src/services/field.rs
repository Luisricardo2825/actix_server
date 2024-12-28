use crate::controller::fields::field_controller::FieldController;
use crate::controller::fields::structs::CreateField;
use crate::controller::fields::structs::CreateFieldWithType;
use crate::controller::fields::structs::QueryParams;
use crate::controller::fields::structs::UpdateField;
use crate::routes::utils::reponses::ReturnError;
use crate::utils::get_body::get_body;
use actix_web::web;
use actix_web::Either;
use actix_web::HttpResponse;
use actix_web::Responder;
use actix_web::Result;

pub struct FieldRoute;

impl FieldRoute {
    // Fields routes
    pub async fn create(
        path: web::Path<(String,)>,
        payload: Either<web::Json<CreateFieldWithType>, web::Json<Vec<CreateFieldWithType>>>,
    ) -> Result<impl Responder> {
        let (table_name,) = path.into_inner();
        use std::result::Result::Ok;

        match payload {
            Either::Left(res) => {
                let table = res.into_inner();

                let table = table.to_create_field();
                if table.is_err() {
                    let err = table.err().unwrap();
                    return Ok(HttpResponse::BadRequest().json(err));
                }
                let table = table.unwrap();

                match FieldController::create_field(table_name, table) {
                    Ok(res) => {
                        return Ok(HttpResponse::Created().json(res));
                    }
                    Err(err) => {
                        return Ok(HttpResponse::BadRequest().json(err));
                    }
                }
            }
            Either::Right(res) => {
                let res = res.into_inner();
                let table: Vec<Result<CreateField, ReturnError>> =
                    res.iter().map(|x| x.to_create_field()).collect();

                for ele in &table {
                    let table = ele;

                    if table.is_err() {
                        let err =
                            <std::result::Result<CreateField, ReturnError> as Clone>::clone(&table)
                                .err()
                                .unwrap();
                        return Ok(HttpResponse::BadRequest().json(err));
                    }
                }
                let table: Vec<CreateField> = table.into_iter().map(|x| x.unwrap()).collect();
                match FieldController::create_fields(table_name, table) {
                    Ok(res) => {
                        return Ok(HttpResponse::Created().json(res));
                    }
                    Err(err) => {
                        return Ok(HttpResponse::BadRequest().json(err));
                    }
                }
            }
        }
    }
    pub async fn find_by_name(path: web::Path<(String, String)>) -> Result<impl Responder> {
        let (table_name, field_name) = path.into_inner();
        match FieldController::find_field_by_name(table_name, field_name) {
            Ok(results) => return Ok(HttpResponse::Ok().json(results)),
            Err(err) => {
                return Ok(HttpResponse::NotFound().json(err));
            }
        }
    }
    pub async fn find(path: web::Path<(String, i32)>) -> Result<impl Responder> {
        let (table_name, field_id) = path.into_inner();
        match FieldController::find_field(table_name, field_id) {
            Ok(results) => return Ok(HttpResponse::Ok().json(results)),
            Err(err) => {
                return Ok(HttpResponse::NotFound().json(err));
            }
        }
    }
    pub async fn find_all(
        path: web::Path<(String,)>,
        query_params: web::Query<QueryParams>,
    ) -> Result<impl Responder> {
        let (table_name,) = path.into_inner();

        match FieldController::find_all_fields(table_name, query_params.into_inner()) {
            Ok(results) => return Ok(HttpResponse::Ok().json(results)),
            Err(err) => {
                return Ok(HttpResponse::NotFound().json(err));
            }
        }
    }
    pub async fn delete(
        table_id: web::Path<i32>,
        field_id: web::Path<i32>,
    ) -> Result<impl Responder> {
        let table_id = table_id.into_inner();
        let field_id = field_id.into_inner();
        match FieldController::delete_field(table_id, field_id) {
            Ok(res) => {
                return Ok(HttpResponse::Ok().json(res)); // if Successful, return the deleted data
            }
            Err(err) => {
                let not_found = err.to_string().to_lowercase().contains("not found");
                if not_found {
                    return Ok(HttpResponse::NotFound().json(ReturnError {
                        error_msg: format!("table with id: {} not found", &table_id),
                        values: None,
                    }));
                }
                return Ok(HttpResponse::BadRequest().json(ReturnError {
                    error_msg: err.to_string(),
                    values: None,
                }));
            }
        }
    }
    pub async fn delete_by_name(
        table_id: web::Path<i32>,
        field_name: web::Path<String>,
    ) -> Result<impl Responder> {
        let table_id = table_id.into_inner();
        let field_name = field_name.into_inner();
        let result = match FieldController::delete_field_by_name(table_id, field_name) {
            Ok(res) => {
                HttpResponse::Ok().json(res) // if Successful, return the deleted data
            }
            Err(err) => {
                let not_found = err.to_string().to_lowercase().contains("not found");
                if not_found {
                    HttpResponse::NotFound().json(ReturnError {
                        error_msg: format!("table with id: {} not found", &table_id),
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
    pub async fn update(field_id: web::Path<i32>, payload: web::Payload) -> Result<impl Responder> {
        let field_id = field_id.into_inner();
        let mut table = match get_body::<UpdateField>(payload).await {
            Ok(res) => res,
            Err(err) => return Ok(HttpResponse::BadRequest().json(err)),
        };

        table.updated_at = Some(chrono::Utc::now().naive_utc()); // update the updated_at field with the current time

        match FieldController::update_field(field_id, table) {
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
