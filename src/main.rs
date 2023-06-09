use actix_server::schema::posts::dsl::*;
use actix_server::{controller::db::establish_connection, models::posts_model::Post};
use actix_web::middleware::Logger;
use actix_web::{get, web, App, HttpServer, Responder, Result};
use diesel::prelude::*;
use env_logger::Env;

#[get("/")]
async fn posts_get() -> Result<impl Responder> {
    let connection = &mut establish_connection();

    let results: Vec<Post> = posts
        .filter(published.eq(true)) // Somente post publicados
        .limit(5) // Limite de 5 post
        .load::<Post>(connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());

    Ok(web::Json(results))
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| App::new().wrap(Logger::default()).service(posts_get))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

// cargo watch -c -w src -x run
