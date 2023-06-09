use actix_web::post;
use actix_web::web;
use actix_web::HttpResponse;
use actix_web::Responder;
use actix_web::Result;

use diesel::insert_into;
use diesel::prelude::*;

use futures::StreamExt;

use serde::Deserialize;
use serde::Serialize;

use crate::controller::db::establish_connection;
use crate::schema::posts::dsl::*;

#[derive(Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::posts)]
struct NewPost {
    id: Option<i32>,
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

/** Create a new post

 ### Params:
  > `payload`: [actix_web::web::Payload]
### Payload example:
```json
{
    "id": 7,
    "title": "Why cats don't have wings?",
    "body": "Because they are not a bird, you dummy",
    "published": true
}
```
*/
#[post("/")]
async fn main(mut payload: web::Payload) -> Result<impl Responder> {
    let mut json = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        match chunk {
            Ok(chunk) => {
                if (json.len() + chunk.len()) > MAX_SIZE {
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
        // limit max size of in-memory payload
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
    match insert_into(posts)
        .values(&new_post)
        .returning(id)
        .get_result::<i32>(connection)
    {
        Ok(res) => {
            #[derive(Serialize)]
            struct Return {
                id: i32,
            }

            return Ok(HttpResponse::Created().json(Return { id: res }));
        }
        Err(err) => {
            return Ok(HttpResponse::BadRequest().json(ReturnError {
                error_msg: err.to_string(),
                values: Some(new_post),
            }))
        }
    }
}
