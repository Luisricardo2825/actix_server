use actix_web::Result;

use actix_web::web;

use actix_web::web::BytesMut;
use futures::StreamExt;

use crate::routes::utils::reponses::ReturnError;

const MAX_SIZE: usize = 256_000; // max payload size is 256k

pub(crate) async fn deserialize_payload(
    mut json: BytesMut,
    mut payload: web::Payload,
) -> Result<BytesMut, ReturnError<String>> {
    while let Some(chunk) = payload.next().await {
        match chunk {
            Ok(chunk) => {
                if (json.len() + chunk.len()) > MAX_SIZE {
                    // limit max size of in-memory payload
                    return Err(ReturnError {
                        error_msg: String::from("Request body overflow"),
                        values: None,
                    });
                }
                json.extend_from_slice(&chunk);
            }
            Err(err) => {
                return Err(ReturnError {
                    error_msg: err.to_string(),
                    values: None,
                });
            }
        }
    }
    Ok(json)
}
