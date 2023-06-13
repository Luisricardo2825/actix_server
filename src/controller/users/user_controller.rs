use actix_web::Result;

use diesel::delete;
use diesel::insert_into;
use diesel::prelude::*;
use diesel::update;

use crate::models::db::connection::establish_connection;
use crate::models::users_model::User;
use crate::routes::utils::reponses::ReturnError;
use crate::schema::users::dsl;

use super::structs::Create;
use super::structs::QueryParams;
use super::structs::Update;
use super::utils::password::PasswordUtils;

pub struct UserController;

impl UserController {
    pub fn delete(id: i32) -> Result<User, ReturnError<i32>> {
        let connection = &mut establish_connection();
        match delete(dsl::users)
            .filter(dsl::id.eq(id))
            .get_result::<User>(connection)
        {
            Ok(res) => {
                return Ok(res); // if Successful, return the deleted data
            }
            Err(err) => Err(ReturnError::<i32> {
                error_msg: err.to_string(),
                values: Some(id),
            }),
        }
    }
    pub fn create(mut new_user: Create) -> Result<User, ReturnError<Create>> {
        let connection = &mut establish_connection();

        // Check if user already exists
        if user_exists(&new_user.email) {
            return Err(ReturnError::<Create> {
                error_msg: format!("A user with email: {}, already exists", new_user.email),
                values: Some(new_user),
            });
        }

        new_user = PasswordUtils::hash(new_user); // Hash password
        let query = insert_into(dsl::users).values(&new_user);

        match query.get_result::<User>(connection) {
            Ok(res) => {
                return Ok(res);
            }
            Err(err) => {
                return Err(ReturnError::<Create> {
                    error_msg: err.to_string(),
                    values: Some(new_user),
                });
            }
        }
    }
    pub fn create_default_admin(mut new_user: Create) -> bool {
        let connection = &mut establish_connection();

        // Check if user already exists
        if user_exists(&new_user.email) {
            return false;
        }

        new_user = PasswordUtils::hash(new_user); // Hash password
        let query = insert_into(dsl::users).values(&new_user);
        match query.get_result::<User>(connection) {
            Ok(_) => return true,
            Err(_) => false,
        }
    }
    pub fn update(new_user: Update) -> Result<User, ReturnError<Update>> {
        let connection = &mut establish_connection();
        match update(dsl::users)
            .set(&new_user)
            .filter(dsl::id.eq(&new_user.id))
            .get_result::<User>(connection)
        {
            Ok(res) => {
                return Ok(res);
            }
            Err(err) => {
                return Err(ReturnError {
                    error_msg: err.to_string(),
                    values: Some(new_user),
                });
            }
        }
    }
    pub fn find_all(query_params: QueryParams) -> Result<Vec<User>, ReturnError<QueryParams>> {
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
            Ok(results) => return Ok(results),
            Err(err) => {
                return Err(ReturnError::<QueryParams> {
                    error_msg: err.to_string(),
                    values: Some(query_params),
                });
            }
        }
    }
    pub fn find(user_id: i32) -> Result<User, ReturnError<i32>> {
        let connection: &mut PgConnection = &mut establish_connection();
        let mut query = dsl::users.into_boxed();
        query = query.filter(dsl::id.eq(user_id)); // Search for a unique user
        match query.first::<User>(connection) {
            Ok(results) => return Ok(results),
            Err(err) => {
                return Err(ReturnError::<i32> {
                    error_msg: err.to_string(),
                    values: Some(user_id),
                });
            }
        }
    }
    pub fn find_by_email(user_email: String) -> Result<User, ReturnError<String>> {
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
