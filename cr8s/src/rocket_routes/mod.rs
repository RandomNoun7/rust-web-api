pub mod crates;
pub mod rustaceans;

use rocket_db_pools::{self, diesel::PgPool, Database};

#[derive(Database)]
#[database("postgres")]
pub struct DbConn(PgPool);
