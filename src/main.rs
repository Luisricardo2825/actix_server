use actix_server::config::query_cfg;

use actix_server::controller::users::structs::Create;
use actix_server::controller::users::user_controller;
use actix_server::middlewares::CHECK_LOGIN;
use actix_server::routes::scopes::Scopes;

use actix_web::middleware::{Compress, DefaultHeaders, Logger, NormalizePath};
use actix_web::{App, HttpServer};
use env_logger::Env;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    if init_server().await {
        info!("Created default user!");
    }
    HttpServer::new(move || {
        App::new()
            .app_data(query_cfg::main())
            .wrap(Logger::default())
            .wrap(DefaultHeaders::new().add(("Content-Type", "application/json")))
            .wrap(Compress::default())
            .wrap(NormalizePath::new(
                actix_web::middleware::TrailingSlash::Always,
            )) // Normalize trailing slash(Resolve the "/" at ending of a endpoint)
            .service(Scopes::posts_scope().wrap(CHECK_LOGIN))
            .service(Scopes::users_scope().wrap(CHECK_LOGIN))
            .service(Scopes::login_scope())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
use log::info;

async fn init_server() -> bool {
    let created = user_controller::UserController::create_default_admin({
            Create {
                id: Some(0),
                name: "admin".to_string(),
                email: "admin@adm.com".to_string(),
                password: "admin".to_string(),
                blocked: None,
                admin: Some(true),
                api_rights: Some(true),
                created_at: None,
                updated_at: None,
            }
        });
        created
}
// cargo watch -c -w src -x run
//     let debug = diesel::debug_query::<diesel::pg::Pg, _>(&query);
