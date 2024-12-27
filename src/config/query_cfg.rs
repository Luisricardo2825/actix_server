use actix_web::{error, web::QueryConfig, HttpResponse};

use crate::routes::utils::reponses::ReturnError;

// Used to change the default error when a query is invalid
pub fn main() -> QueryConfig {
    QueryConfig::default()
        // use custom error handler
        .error_handler(|err: error::QueryPayloadError, req| {
            let erro = err.to_string(); // Necessary to clone the error and show it in the error reponse as string
            error::InternalError::from_response(
                erro.clone(),
                HttpResponse::BadRequest().json(ReturnError {
                    error_msg: erro,
                    values: Some(req.query_string().into()),
                }),
            )
            .into()
        })
}
