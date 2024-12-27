use std::future::{ready, Ready};

use actix_web::{
    body::EitherBody,
    dev::{self, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage, HttpResponse,
};
use futures_util::future::LocalBoxFuture;

use crate::{
    controller::login::auth_controller::AuthController, routes::utils::reponses::ReturnError,
};

pub struct CheckLogin;

impl<S, B> Transform<S, ServiceRequest> for CheckLogin
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = CheckLoginMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(CheckLoginMiddleware { service }))
    }
}
pub struct CheckLoginMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for CheckLoginMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    dev::forward_ready!(service);

    fn call(&self, request: ServiceRequest) -> Self::Future {
        // Check if has token
        let is_logged_in = request.headers().get("Authorization").is_some();
        let mut error_ret = ReturnError {
            error_msg: "Token missing".to_string(),
            values: None,
        };
        // Check if token is missing
        if !is_logged_in {
            let (request, _pl) = request.into_parts();
            // Get path variable
            let path = request.path().to_string();
            let path = path.split("/").filter(|x| !x.is_empty()).last().unwrap();

            error_ret.error_msg = "Token missing".to_string();
            error_ret.values = Some(path.into());
            let response = HttpResponse::Unauthorized()
                .json(error_ret)
                // constructed responses map to "right" body
                .map_into_right_body();

            return Box::pin(async { Ok(ServiceResponse::new(request, response)) });
        }
        let token = request
            .headers()
            .get("Authorization")
            .unwrap()
            .to_str()
            .expect("Error getting token");

        let (authorized, claims) = AuthController::verify_jwt(token.to_owned());

        if !authorized {
            let (request, _pl) = request.into_parts();
            error_ret.error_msg = "Token invalid".to_string();
            let response = HttpResponse::Unauthorized()
                .json(error_ret)
                // constructed responses map to "right" body
                .map_into_right_body();

            return Box::pin(async { Ok(ServiceResponse::new(request, response)) });
        }

        let claims = claims.unwrap();
        if !claims.api_rights {
            let (request, _pl) = request.into_parts();
            error_ret.error_msg = "Not authorized".to_string();
            let response = HttpResponse::Unauthorized()
                .json(error_ret)
                // constructed responses map to "right" body
                .map_into_right_body();

            return Box::pin(async { Ok(ServiceResponse::new(request, response)) });
        }

        request.extensions_mut().insert(claims);

        let res = self.service.call(request);

        Box::pin(async move {
            // forwarded responses map to "left" body
            res.await.map(ServiceResponse::map_into_left_body)
        })
    }
}
