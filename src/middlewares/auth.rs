use actix_web::{dev::ServiceRequest, error, Error};
use actix_web_httpauth::extractors::bearer::BearerAuth;

use crate::routes::utils::reponses::ReturnError;

pub async fn bearer(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    if credentials.token() == "mF_9.B5f-4.1JqM" {
        Ok(req)
    } else {
        // Use default implementation for `error_response()` method
        impl error::ResponseError for ReturnError<String> {}
        let error_custom = ReturnError::<String> {
            error_msg: "Token invalid ".to_string(),
            values: None,
        };

        Err((error_custom.into(), req))
    }
}
