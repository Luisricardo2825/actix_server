use actix_web::{dev::ServiceRequest, Error};
use actix_web_httpauth::extractors::basic::BasicAuth;
use actix_web_httpauth::extractors::bearer::BearerAuth;

pub async fn basic(
    req: ServiceRequest,
    credentials: BasicAuth,
) -> Result<ServiceRequest, (actix_web::error::Error, ServiceRequest)> {
    eprintln!("{credentials:?}");
    // All users are great and more than welcome!
    Ok(req)
}

pub async fn bearer(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {

    eprintln!("{:?}", credentials.token());
    Ok(req)
}
