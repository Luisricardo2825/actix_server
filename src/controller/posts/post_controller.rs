use actix_web::web;
use actix_web::HttpResponse;
use actix_web::Responder;
use actix_web::Result;

use diesel::delete;
use diesel::insert_into;
use diesel::prelude::*;
use diesel::update;

use crate::models::db::connection::establish_connection;
use crate::models::posts_model::Post;
use crate::routes::utils::reponses::ReturnError;
use crate::schema::posts::dsl;
use crate::utils::deserialize_payload::deserialize_payload;

use super::structs::Create;
use super::structs::Delete;
use super::structs::QueryParams;
use super::structs::Update;

pub struct PostController;

impl PostController {
    pub async fn delete(payload: web::Payload) -> Result<impl Responder> {
        let mut json = web::BytesMut::new();
        json = match deserialize_payload(json, payload).await {
            Ok(res) => res,
            Err(err) => {
                return Ok(HttpResponse::BadRequest().json(err));
            }
        };

        // body is loaded, now we can deserialize serde-json
        let new_post = match serde_json::from_slice::<Delete>(&json) {
            Ok(res) => res,
            Err(err) => {
                return Ok(HttpResponse::BadRequest().json(ReturnError::<Delete> {
                    error_msg: format!("Invalid JSON: {}", err.to_string()),
                    values: None,
                }));
            }
        };

        let connection = &mut establish_connection();
        match delete(dsl::posts)
            .filter(dsl::id.eq(&new_post.id))
            .get_result::<Post>(connection)
        {
            Ok(res) => {
                return Ok(HttpResponse::Ok().json(res)); // if Successful, return the deleted data
            }
            Err(err) => {
                let not_found = err.to_string().to_lowercase().contains("not found");
                if not_found {
                    return Ok(HttpResponse::NotFound().json(ReturnError::<Delete> {
                        error_msg: format!("Post with id: {} not found", &new_post.id),
                        values: Some(new_post),
                    }));
                }
                return Ok(HttpResponse::BadRequest().json(ReturnError::<Delete> {
                    error_msg: err.to_string(),
                    values: Some(new_post),
                })); // if Successful, return the ID of the inserted post
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
        let new_post = match serde_json::from_slice::<Create>(&json) {
            Ok(res) => res,
            Err(err) => {
                return Ok(HttpResponse::BadRequest().json(ReturnError::<Create> {
                    error_msg: format!("Invalid JSON: {}", err.to_string()),
                    values: None,
                }));
            }
        };

        let connection = &mut establish_connection();
        match insert_into(dsl::posts)
            .values(&new_post)
            .get_result::<Post>(connection)
        {
            Ok(res) => {
                return Ok(HttpResponse::Created().json(res));
                // if Successful, return the ID of the inserted post
            }
            Err(err) => {
                return Ok(HttpResponse::BadRequest().json(ReturnError::<Create> {
                    error_msg: err.to_string(),
                    values: Some(new_post),
                })); // if Successful, return the ID of the inserted post
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
        let new_post = match serde_json::from_slice::<Update>(&json) {
            Ok(res) => res,
            Err(err) => {
                return Ok(HttpResponse::BadRequest().json(ReturnError::<String> {
                    error_msg: format!("Invalid JSON: {}", err.to_string()),
                    values: None,
                }));
            }
        };

        let connection = &mut establish_connection();
        match update(dsl::posts)
            .set(&new_post)
            .filter(dsl::id.eq(&new_post.id))
            .get_result::<Post>(connection)
        {
            Ok(res) => {
                return Ok(HttpResponse::Ok().json(res)); // if Successful, return the ID of the inserted post
            }
            Err(err) => {
                let not_found = err.to_string().to_lowercase().contains("not found");
                if not_found {
                    return Ok(HttpResponse::NotFound().json(ReturnError {
                        error_msg: format!("Post with id: {} not found", &new_post.id),
                        values: Some(new_post),
                    }));
                }
                return Ok(HttpResponse::BadRequest().json(ReturnError {
                    error_msg: err.to_string(),
                    values: Some(new_post),
                })); // if Successful, return the ID of the inserted post
            }
        }
    }
    pub async fn find_all(query_params: web::Query<QueryParams>) -> Result<impl Responder> {
        let connection = &mut establish_connection();
        let mut query = dsl::posts.into_boxed();

        if let Some(id_query) = query_params.id {
            query = query.filter(dsl::id.eq(id_query)); // Search for a unique post
        };
        if let Some(per_page) = query_params.per_page {
            query = query.limit(per_page); // Define user posts per page
        } else {
            query = query.limit(100) // Default limit to 100
        }

        match query.load::<Post>(connection) {
            Ok(results) => return Ok(HttpResponse::Ok().json(results)),
            Err(err) => {
                return Ok(HttpResponse::BadRequest().json(ReturnError::<QueryParams> {
                    error_msg: err.to_string(),
                    values: Some(query_params.0),
                })); // if Successful, return the ID of the inserted post
            }
        }
    }
    pub async fn find(post_id: web::Path<i32>) -> Result<impl Responder> {
        let id = post_id.into_inner();
        let connection: &mut PgConnection = &mut establish_connection();
        let mut query = dsl::posts.into_boxed();
        query = query.filter(dsl::id.eq(id)); // Search for a unique post
        match query.first::<Post>(connection) {
            Ok(results) => return Ok(HttpResponse::Ok().json(results)),
            Err(err) => {
                return Ok(HttpResponse::NotFound().json(ReturnError::<i32> {
                    error_msg: err.to_string(),
                    values: Some(id),
                })); // if Successful, return the ID of the inserted post
            }
        }
    }
}
