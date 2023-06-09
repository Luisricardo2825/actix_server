use actix_web::web;
use actix_web::Responder;
use actix_web::Result;
use actix_web::get;

use diesel::prelude::*;

use crate::schema::posts::dsl::*;
use crate::{controller::db::establish_connection, models::posts_model::Post};



#[get("/")]
async fn main() -> Result<impl Responder> {
    let connection = &mut establish_connection();

    let results: Vec<Post> = posts
        .filter(published.eq(true)) // Somente post publicados
        .limit(5) // Limite de 5 post
        .load::<Post>(connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());

    Ok(web::Json(results))
}
