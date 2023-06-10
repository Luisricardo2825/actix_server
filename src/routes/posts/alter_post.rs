use actix_web::patch;
use actix_web::web;
use actix_web::HttpResponse;
use actix_web::Responder;
use actix_web::Result;

use diesel::prelude::*;
use diesel::update;

use futures::StreamExt;

use serde::Deserialize;
use serde::Serialize;

use crate::controller::db::establish_connection;
use crate::models::posts_model::Post;
use crate::schema::posts::dsl::*;

#[derive(Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = crate::schema::posts)]
struct NewPost {
    id: i32,
    title: Option<String>,
    body: Option<String>,
    published: Option<bool>,
}

#[derive(Serialize)]
struct ReturnError {
    error_msg: String,
    values: Option<NewPost>,
}
const MAX_SIZE: usize = 256_000; // max payload size is 256k

/** Alter a post. Obs: All props, except `id` are optional

 ### Params:
  > `payload`: [actix_web::web::Payload]
### Payload example
```json
{
    "id": 7,
    "title": "Why cats don't have wings?",
}
```
*/
#[patch("/")]
async fn main(mut payload: web::Payload) -> Result<impl Responder> {
    let mut json = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        match chunk {
            Ok(chunk) => {
                if (json.len() + chunk.len()) > MAX_SIZE {
                    // limit max size of in-memory payload
                    return Ok(HttpResponse::BadRequest().json(ReturnError {
                        error_msg: String::from("Request body overflow"),
                        values: None,
                    }));
                }
                json.extend_from_slice(&chunk);
            }
            Err(err) => {
                return Ok(HttpResponse::BadRequest().json(ReturnError {
                    error_msg: err.to_string(),
                    values: None,
                }));
            }
        }
    }

    // body is loaded, now we can deserialize serde-json
    let new_post = match serde_json::from_slice::<NewPost>(&json) {
        Ok(res) => res,
        Err(err) => {
            return Ok(HttpResponse::BadRequest().json(ReturnError {
                error_msg: format!("Invalid JSON: {}", err.to_string()),
                values: None,
            }));
        }
    };

    let connection = &mut establish_connection();
    match update(posts)
        .set(&new_post)
        .filter(id.eq(&new_post.id))
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
