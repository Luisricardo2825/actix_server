use std::env;

use actix_web::rt;
use dotenvy::dotenv;
use tokio_postgres::{Client, Error, NoTls};

pub async fn establish_driver_connection() -> Result<Client, Error> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let (client, connection) = tokio_postgres::connect(&database_url, NoTls).await?;

    rt::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    Ok(client)
}
