use serde_json::{json, Value};
use std::error::Error;
pub mod crates;
pub mod rustaceans;

use rocket::{http::Status, response::status::Custom};
use rocket_db_pools::{self, diesel::PgPool, Database};

#[derive(Database)]
#[database("postgres")]
pub struct DbConn(PgPool);

pub fn server_error(e: Box<dyn Error>) -> Custom<Value> {
    rocket::error!("{}", e);
    Custom(Status::InternalServerError, json!("Error"))
}
