use actix_web::Result;

use diesel::delete;
use diesel::insert_into;
use diesel::prelude::*;
use diesel::update;

use crate::controller::QueryParams;
use crate::controller::API_LIMIT;
use crate::models::db::connection::establish_connection;
use crate::models::posts_model::Post;
use crate::routes::utils::reponses::ReturnError;
use crate::schema::posts::dsl;

use super::structs::Create;
use super::structs::Update;

pub struct PostController;

impl PostController {
    pub fn delete(id: i32) -> Result<Post, ReturnError> {
        let connection = &mut establish_connection();
        match delete(dsl::posts)
            .filter(dsl::id.eq(&id))
            .get_result(connection)
        {
            Ok(res) => {
                return Ok(res); // if Successful, return the deleted data
            }
            Err(err) => {
                return Err(ReturnError::new(err.to_string(), id)); // if Successful, return the ID of the inserted post
            }
        }
    }
    pub fn create(new_post: Create) -> Result<Post, ReturnError> {
        let connection = &mut establish_connection();
        match insert_into(dsl::posts)
            .values(&new_post)
            .get_result(connection)
        {
            Ok(res) => {
                return Ok(res);
                // if Successful, return the ID of the inserted post
            }
            Err(err) => {
                return Err(ReturnError::new(err.to_string(), new_post)); // if Successful, return the ID of the inserted post
            }
        }
    }
    pub fn update(post_id: i32, new_post: Update) -> Result<Post, ReturnError> {
        let connection = &mut establish_connection();
        match update(dsl::posts)
            .set(&new_post)
            .filter(dsl::id.eq(post_id))
            .get_result(connection)
        {
            Ok(res) => {
                return Ok(res); // if Successful, return the ID of the inserted post
            }
            Err(err) => {
                return Err(ReturnError::new(err.to_string(), new_post)); // if Successful, return the ID of the inserted post
            }
        }
    }
    pub fn find_all(query_params: QueryParams) -> Result<Vec<Post>, ReturnError> {
        let connection = &mut establish_connection();
        let mut query = dsl::posts.into_boxed();

        if let Some(id_query) = query_params.id {
            query = query.filter(dsl::id.eq(id_query)); // Search for a unique post
        };
        if let Some(limit) = query_params.limit {
            query = query.limit(limit); // Define user posts per page
        } else {
            query = query.limit(API_LIMIT) // Default limit
        }

        match query.load(connection) {
            Ok(results) => return Ok(results),
            Err(err) => {
                return Err(ReturnError::new(err.to_string(), query_params)); // if Successful, return the ID of the inserted post
            }
        }
    }
    pub fn find(id: i32) -> Result<Post, ReturnError> {
        let connection: &mut PgConnection = &mut establish_connection();
        let mut query = dsl::posts.into_boxed();
        query = query.filter(dsl::id.eq(id)); // Search for a unique post
        match query.first(connection) {
            Ok(results) => return Ok(results),
            Err(err) => {
                return Err(ReturnError::new(err.to_string(), id)); // if Successful, return the ID of the inserted post
            }
        }
    }
}
