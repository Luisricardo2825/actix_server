use std::sync::Arc;

use actix_web::{error, web::QueryConfig, HttpResponse};

use crate::routes::utils::reponses::ReturnError;

pub fn main() -> QueryConfig {
    QueryConfig::default()
        // use custom error handler
        .error_handler(|err: error::QueryPayloadError, req| {
            let erro = Arc::from(err); // Necessary to clone the error and show it in the error reponse as string
            error::InternalError::from_response(
                erro.clone(),
                HttpResponse::BadRequest().json(ReturnError::<&str> {
                    error_msg: erro.to_string(),
                    values: Some(req.query_string()),
                }),
            )
            .into()
        })
}
