use std::error::Error;

use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::{Value,json};

pub mod rustaceans;
pub mod crates;

#[derive(rocket_db_pools::Database)]
#[database("postgres")]
pub struct DbConn(rocket_db_pools::diesel::PgPool);


pub fn server_error(e: Box<dyn Error>) -> Custom<Value> {
    rocket::error!("{}",e);
    Custom(Status::InternalServerError,json!("Error"))
}