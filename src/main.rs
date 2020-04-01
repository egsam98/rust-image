#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use rocket::State;
use rocket::http::{Status, ContentType};
use crate::services::image_service::ImageService;
use crate::db::{SqlitePool};
use rocket::response::Response;
use rocket_contrib::json::{JsonValue};
use crate::utils::images_multipart::ImagesMultipart;
use std::str::FromStr;
use rocket::http::RawStr;
use crate::utils::http::{error_json_response, raw_response};

mod db;
mod models;
mod services;
mod schema;
mod utils;


#[get("/<id>?<preview>")]
fn get_image<'r>(id: &RawStr, preview: Option<bool>, pool: State<SqlitePool>) -> Response<'r> {
    match id.as_str().parse() {
        Err(_) => error_json_response("Image id must be integer", Status::BadRequest),
        Ok(id) => match ImageService::find(id, pool.inner()) {
            Err(_) => error_json_response("Image not found", Status::NotFound),
            Ok(image) => {
                let bytes = match preview.unwrap_or(false) {
                    true => image.preview_bytes.expect("Image preview must be set, \
                        ImageService's internal error"),
                    false => image.bytes
                };
                let content_type = ContentType::from_str(image.content_type.as_str());
                raw_response(bytes, content_type.unwrap())
            }
        }
    }
}

#[post("/", data="<images_multipart>")]
fn create_image<'r>(images_multipart: ImagesMultipart, pool: State<SqlitePool>) -> JsonValue {
    let new_ids: Vec<i32> = images_multipart.images.iter().map(|image_form| {
        ImageService::upload(image_form, pool.inner())
    }).collect();
    return json!({"ids": new_ids});
}

fn main() {
    rocket::ignite()
        .manage(SqlitePool::new())
        .mount("/api/image", routes![get_image, create_image])
        .launch();
}