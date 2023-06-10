use actix_web::get;
use actix_web::web;

use actix_web::HttpResponse;
use actix_web::Responder;
use actix_web::Result;

use diesel::prelude::*;
use serde::Deserialize;
use serde::Serialize;

use crate::routes::utils::reponses::ReturnError;
use crate::schema::posts::dsl::*;
use crate::{controller::db::establish_connection, models::posts_model::Post};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct QueryParams {
    id: Option<i32>,
    per_page: Option<i64>,
}

#[get("/")]
async fn main(query_params: web::Query<QueryParams>) -> Result<impl Responder> {
    let connection = &mut establish_connection();
    let mut query = posts.into_boxed();

    if let Some(id_query) = query_params.id {
        query = query.filter(id.eq(id_query)); // Search for a unique post
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
