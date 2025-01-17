use crate::models::{NewRustacean, Rustacean};
use crate::rocket_routes::DbConn;
use crate::{models, repositories::RustaceanRepository};
use diesel::prelude::*;
use rocket::response::status::NoContent;
use rocket::serde::json::Json;
use rocket::{
    http::Status,
    response::status::{self, Custom},
    serde::json::{self, json},
};
use rocket_db_pools::Connection;

#[rocket::get("/rustaceans")]
pub async fn get_rustaceans(
    mut db: Connection<DbConn>,
) -> Result<json::Value, Custom<json::Value>> {
    RustaceanRepository::find_multiple(&mut db, 100)
        .await
        .map(|rustaceans| json!(rustaceans))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
}

#[rocket::get("/rustaceans/<id>")]
pub async fn view_rustacean(
    mut db: Connection<DbConn>,
    id: i32,
) -> Result<json::Value, Custom<json::Value>> {
    RustaceanRepository::find(&mut db, id)
        .await
        .map(|rustacean| json!(rustacean))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
}

#[rocket::post("/rustaceans", format = "json", data = "<new_rustacean>")]
pub async fn create_rustacean(
    mut db: Connection<DbConn>,
    new_rustacean: Json<NewRustacean>,
) -> Result<Custom<json::Value>, Custom<json::Value>> {
    RustaceanRepository::create(&mut db, new_rustacean.into_inner())
        .await
        .map(|rustacean| Custom(Status::Created, json!(rustacean)))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
}

#[rocket::put("/rustaceans/<id>", format = "json", data = "<new_rustacean>")]
pub async fn update_rustacean(
    mut db: Connection<DbConn>,
    id: i32,
    new_rustacean: Json<NewRustacean>,
) -> Result<json::Value, Custom<json::Value>> {
    RustaceanRepository::update(&mut db, id, new_rustacean.into_inner())
        .await
        .map(|rustacean| json!(rustacean))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
}

#[rocket::delete("/rustaceans/<id>")]
pub async fn delete_rustacean(
    mut db: Connection<DbConn>,
    id: i32,
) -> Result<NoContent, Custom<json::Value>> {
    RustaceanRepository::delete(&mut db, id)
        .await
        .map(|_| NoContent)
        .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
}
