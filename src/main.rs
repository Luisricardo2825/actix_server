use actix_server::config::query_cfg;
use actix_server::middlewares::auth::bearer;
use actix_server::routes::scopes::{posts_route, users_route};
use actix_web::middleware::{Compress, Logger, NormalizePath};
use actix_web::{App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use env_logger::Env;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
            .app_data(query_cfg::main())
            .wrap(Logger::default())
            .wrap(Compress::default())
            .wrap(NormalizePath::new(
                actix_web::middleware::TrailingSlash::Always,
            )) // Normalize trailing slash(Resolve the "/" at ending of a endpoint)
            .wrap(HttpAuthentication::bearer(bearer))
            .service(posts_route())
            .service(users_route())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

// cargo watch -c -w src -x run
