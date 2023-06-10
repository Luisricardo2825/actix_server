use std::sync::Arc;

use actix_server::routes::scopes::posts_route;
use actix_server::routes::utils::reponses::ReturnError;
use actix_web::middleware::{Compress, Logger, NormalizePath};
use actix_web::{error, web, HttpResponse};
use actix_web::{App, HttpServer};
use env_logger::Env;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let query_cfg = web::QueryConfig::default()
        // use custom error handler
        .error_handler(|err: error::QueryPayloadError, req| {
            let erro = Arc::from(err); // Necessary to clone the error and show it in the error reponse as string
            error::InternalError::from_response(
                erro.clone(),
                HttpResponse::Conflict().json(ReturnError::<&str> {
                    error_msg: erro.to_string(),
                    values: Some(req.query_string()),
                }),
            )
            .into()
        });

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
            .app_data(query_cfg.to_owned())
            .wrap(Logger::default())
            .wrap(Compress::default())
            .wrap(NormalizePath::new(
                actix_web::middleware::TrailingSlash::Always,
            )) // Normalize trailing slash(Resolve the "/" at ending of a endpoint)
            .service(posts_route())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

// cargo watch -c -w src -x run
