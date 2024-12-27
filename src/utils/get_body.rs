use actix_web::Result;

use actix_web::web;

use actix_web::web::BytesMut;
use futures::StreamExt;
use serde::de::DeserializeOwned;

use crate::routes::utils::reponses::ReturnError;

const MAX_SIZE: usize = 256_000; // max payload size is 256k

pub(crate) async fn get_body<T: DeserializeOwned>(payload: web::Payload) -> Result<T, ReturnError> {
    let mut json = web::BytesMut::new();
    json = match deserialize_payload(json, payload).await {
        Ok(res) => res,
        Err(err) => return Err(err),
    };

    // body is loaded, now we can deserialize serde-json
    let request_body = match serde_json::from_slice::<T>(&json) {
        Ok(res) => res,
        Err(err) => {
            return Err(ReturnError::without_value(format!(
                "Invalid JSON: {}",
                err.to_string()
            )));
        }
    };

    Ok(request_body)
}

async fn deserialize_payload(
    mut json: BytesMut,
    mut payload: web::Payload,
) -> Result<BytesMut, ReturnError> {
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
                return Err(ReturnError::without_value(err.to_string()));
            }
        }
    }
    Ok(json)
}
