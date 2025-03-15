use crate::models::NewCrate;
use crate::rocket_routes::{server_error, DbConn};
use crate::repositories::CrateRepository;
use rocket::{
    http::Status,
    response::status::{Custom, NoContent},
    serde::json::{self, json, Json},
};

use rocket_db_pools::Connection;

#[rocket::get("/crates")]
pub async fn get_crates(mut db: Connection<DbConn>) -> Result<json::Value, Custom<json::Value>> {
    CrateRepository::find_multiple(&mut db, 100)
        .await
        .map(|crates| json!(crates))
        .map_err(|e| server_error(e.into()))
}

#[rocket::get("/crates/<id>")]
pub async fn view_crates(
    mut db: Connection<DbConn>,
    id: i32,
) -> Result<json::Value, Custom<json::Value>> {
    CrateRepository::find(&mut db, id)
        .await
        .map(|krate| json!(krate))
        .map_err(|e| server_error(e.into()))
}

#[rocket::post("/crates", format = "json", data = "<new_crate>")]
pub async fn create_crate(
    mut db: Connection<DbConn>,
    new_crate: Json<NewCrate>,
) -> Result<Custom<json::Value>, Custom<json::Value>> {
    CrateRepository::create(&mut db, new_crate.into_inner())
        .await
        .map(|krate| Custom(Status::Created, json!(krate)))
        .map_err(|e| server_error(e.into()))
}

#[rocket::put("/crates/<id>", format = "json", data = "<new_crate>")]
pub async fn update_crate(
    mut db: Connection<DbConn>,
    id: i32,
    new_crate: Json<NewCrate>,
) -> Result<json::Value, Custom<json::Value>> {
    CrateRepository::update(&mut db, id, new_crate.into_inner())
        .await
        .map(|krate| json!(krate))
        .map_err(|e| server_error(e.into()))
}

#[rocket::delete("/crates/<id>")]
pub async fn delete_crate(
    mut db: Connection<DbConn>,
    id: i32,
) -> Result<NoContent, Custom<json::Value>> {
    CrateRepository::delete(&mut db, id)
        .await
        .map(|_| NoContent)
        .map_err(|e| server_error(e.into()))
}
