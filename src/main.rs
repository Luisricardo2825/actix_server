use actix_server::routes::posts::{create_post, get_posts};
use actix_web::middleware::{Compress, Logger};
use actix_web::{App, HttpServer};
use env_logger::Env;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Compress::default())
            .service(get_posts::main)
            .service(create_post::main)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

// cargo watch -c -w src -x run
