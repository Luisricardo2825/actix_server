use actix_server::config::query_cfg;

use actix_server::middlewares::CHECK_LOGIN;
use actix_server::routes::scopes::{login_route, posts_route, users_route};

use actix_web::middleware::{Compress, DefaultHeaders, Logger, NormalizePath};
use actix_web::{App, HttpServer};
use env_logger::Env;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
            .app_data(query_cfg::main())
            .wrap(Logger::default())
            .wrap(DefaultHeaders::new().add(("Content-Type", "application/json")))
            .wrap(Compress::default())
            .wrap(NormalizePath::new(
                actix_web::middleware::TrailingSlash::Always,
            )) // Normalize trailing slash(Resolve the "/" at ending of a endpoint)
            .service(posts_route().wrap(CHECK_LOGIN))
            .service(users_route().wrap(CHECK_LOGIN))
            .service(login_route())
        // .wrap(HttpAuthentication::bearer(bearer))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

// cargo watch -c -w src -x run
