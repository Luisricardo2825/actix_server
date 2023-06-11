use actix_web::web;
use actix_web::HttpResponse;
use actix_web::Responder;
use actix_web::Result;

use diesel::delete;
use diesel::insert_into;
use diesel::prelude::*;
use diesel::update;

use crate::models::db::connection::establish_connection;
use crate::models::users_model::User;
use crate::routes::utils::reponses::ReturnError;
use crate::schema::users::dsl;
use crate::utils::deserialize_payload::deserialize_payload;

use super::structs::Create;
use super::structs::Delete;
use super::structs::QueryParams;
use super::structs::Update;
use super::utils::password::PasswordUtils;

pub struct UserController;

impl UserController {
    pub async fn delete(payload: web::Payload) -> Result<impl Responder> {
        let mut json = web::BytesMut::new();
        json = match deserialize_payload(json, payload).await {
            Ok(res) => res,
            Err(err) => {
                return Ok(HttpResponse::BadRequest().json(err));
            }
        };

        // body is loaded, now we can deserialize serde-json
        let new_user = match serde_json::from_slice::<Delete>(&json) {
            Ok(res) => res,
            Err(err) => {
                return Ok(HttpResponse::BadRequest().json(ReturnError::<Delete> {
                    error_msg: format!("Invalid JSON: {}", err.to_string()),
                    values: None,
                }));
            }
        };

        let connection = &mut establish_connection();
        match delete(dsl::users)
            .filter(dsl::id.eq(&new_user.id))
            .get_result::<User>(connection)
        {
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
        let mut json = web::BytesMut::new();
        json = match deserialize_payload(json, payload).await {
            Ok(res) => res,
            Err(err) => {
                return Ok(HttpResponse::BadRequest().json(err));
            }
        };

        // body is loaded, now we can deserialize serde-json
        let mut new_user = match serde_json::from_slice::<Create>(&json) {
            Ok(res) => res,
            Err(err) => {
                return Ok(HttpResponse::BadRequest().json(ReturnError::<Create> {
                    error_msg: format!("Invalid JSON: {}", err.to_string()),
                    values: None,
                }));
            }
        };

        let connection = &mut establish_connection();

        // Check if user already exists
        if user_exists(&new_user.email) {
            return Ok(HttpResponse::Found().json(ReturnError::<Create> {
                error_msg: format!("A user with email: {}, already exists", new_user.email),
                values: Some(new_user),
            }));
        }

        new_user = PasswordUtils::hash(new_user); // Hash password

        match insert_into(dsl::users)
            .values(&new_user)
            .get_result::<User>(connection)
        {
            Ok(res) => {
                return Ok(HttpResponse::Created().json(res));
            }
            Err(err) => {
                return Ok(HttpResponse::BadRequest().json(ReturnError::<Create> {
                    error_msg: err.to_string(),
                    values: Some(new_user),
                }));
            }
        }
    }
    pub async fn update(payload: web::Payload) -> Result<impl Responder> {
        // Custom update struct

        let mut json = web::BytesMut::new();

        json = match deserialize_payload(json, payload).await {
            Ok(res) => res,
            Err(err) => {
                return Ok(HttpResponse::BadRequest().json(err));
            }
        };
        // body is loaded, now we can deserialize serde-json
        let new_user = match serde_json::from_slice::<Update>(&json) {
            Ok(res) => res,
            Err(err) => {
                return Ok(HttpResponse::BadRequest().json(ReturnError::<String> {
                    error_msg: format!("Invalid JSON: {}", err.to_string()),
                    values: None,
                }));
            }
        };

        let connection = &mut establish_connection();
        match update(dsl::users)
            .set(&new_user)
            .filter(dsl::id.eq(&new_user.id))
            .get_result::<User>(connection)
        {
            Ok(res) => {
                return Ok(HttpResponse::Ok().json(res));
            }
            Err(err) => {
                let not_found = err.to_string().to_lowercase().contains("not found");
                if not_found {
                    return Ok(HttpResponse::NotFound().json(ReturnError {
                        error_msg: format!("User with id: {} not found", &new_user.id),
                        values: Some(new_user),
                    }));
                }
                return Ok(HttpResponse::BadRequest().json(ReturnError {
                    error_msg: err.to_string(),
                    values: Some(new_user),
                }));
            }
        }
    }
    pub async fn find_all(query_params: web::Query<QueryParams>) -> Result<impl Responder> {
        let connection = &mut establish_connection();
        let mut query = dsl::users.into_boxed();

        if let Some(id_query) = query_params.id {
            query = query.filter(dsl::id.eq(id_query));
        };
        if let Some(per_page) = query_params.per_page {
            query = query.limit(per_page);
        } else {
            query = query.limit(100) // Default limit to 100
        }

        match query.load::<User>(connection) {
            Ok(results) => return Ok(HttpResponse::Ok().json(results)),
            Err(err) => {
                return Ok(HttpResponse::BadRequest().json(ReturnError::<QueryParams> {
                    error_msg: err.to_string(),
                    values: Some(query_params.0),
                }));
            }
        }
    }
    pub async fn find(user_id: web::Path<i32>) -> Result<impl Responder> {
        let id = user_id.into_inner();
        let connection: &mut PgConnection = &mut establish_connection();
        let mut query = dsl::users.into_boxed();
        query = query.filter(dsl::id.eq(id)); // Search for a unique user
        match query.first::<User>(connection) {
            Ok(results) => return Ok(HttpResponse::Ok().json(results)),
            Err(err) => {
                return Ok(HttpResponse::NotFound().json(ReturnError::<i32> {
                    error_msg: err.to_string(),
                    values: Some(id),
                }));
            }
        }
    }
    pub async fn find_by_email(user_email: String) -> Result<User, ReturnError<String>> {
        let connection: &mut PgConnection = &mut establish_connection();
        let mut query = dsl::users.into_boxed();
        query = query.filter(dsl::email.eq(&user_email)); // Search for a unique user
        match query.first::<User>(connection) {
            Ok(results) => return Ok(results),
            Err(err) => {
                return Err(ReturnError::<String> {
                    error_msg: err.to_string(),
                    values: Some(user_email),
                })
            }
        }
    }
}

fn user_exists(email: &str) -> bool {
    let connection: &mut PgConnection = &mut establish_connection();
    let mut query = dsl::users.into_boxed();
    query = query.filter(dsl::email.eq(email)); // Search for a unique user
    match query.first::<User>(connection) {
        Ok(_) => return true,
        Err(_) => return false,
    }
}
