use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2;
use dotenvy::dotenv;
use log::error;
use std::env;

use crate::routes::utils::reponses::ReturnError;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn db_poll() -> DbPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = r2d2::ConnectionManager::<diesel::PgConnection>::new(database_url);

    r2d2::Pool::builder()
        .max_size(16)
        .error_handler(Box::new(ReturnError::default()))
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool")
}

pub type DbPool = r2d2::Pool<r2d2::ConnectionManager<diesel::PgConnection>>;

impl<E> r2d2::HandleError<E> for ReturnError
where
    E: std::error::Error,
{
    fn handle_error(&self, error: E) {
        error!("{}", error);
    }
}
