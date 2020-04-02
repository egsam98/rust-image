#![feature(proc_macro_hygiene, decl_macro)]
#![feature(try_trait)]

#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use rocket::State;
use rocket::http::{Status, ContentType};
use crate::services::image_service::ImageService;
use crate::db::{SqlitePool};
use rocket::response::Response;
use rocket_contrib::json::{Json};
use crate::utils::images_multipart::ImagesMultipart;
use std::str::FromStr;
use rocket::http::RawStr;
use crate::utils::http::{raw_response, ToResponse};
use tokio;
use std::collections::HashMap;

mod db;
mod models;
mod services;
mod schema;
#[macro_use]mod utils;


#[get("/<id>?<preview>")]
fn get_image<'r>(id: &RawStr, preview: Option<bool>, pool: State<SqlitePool>) -> Response<'r> {
    let id= match id.as_str().parse() {
        Err(_) => return json_response!(400, {"error": "Image id must be integer"}),
        Ok(id) => id
    };
    match ImageService::find(id, pool.inner()) {
        Err(_) => return json_response!(404, {"error": "Image not found"}),
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

#[post("/", format="multipart/form-data", data="<images_multipart>")]
fn create_image_from_multipart<'r>(images_multipart: ImagesMultipart, pool: State<SqlitePool>) -> Response<'r> {
    let mut new_ids = Vec::new();
    for image_form in images_multipart.images.iter() {
        match ImageService::upload(image_form, pool.inner()) {
            Err(e) => return json_response!(400, {"error": e}),
            Ok(id) => new_ids.push(id)
        }
    };
    return json_response!({"ids": new_ids});
}

#[post("/", format="application/json", data="<data>")]
#[tokio::main]
async fn create_image_from_json<'r>(data: Json<HashMap<&str, &str>>, pool: State<SqlitePool>) -> Response<'r> {
    if let Some(url) = data.get("url") {
        return match ImageService::upload_from_url(url, &pool.inner()).await {
            Ok(id) => json_response!({"id": id}),
            Err(e) => json_response!(400, {"error": e})
        };
    }
    json_response!(400, {"error": "Must be any of json keys: [url]"})
}

fn main() {
    rocket::ignite()
        .manage(SqlitePool::new())
        .mount("/api/image", routes![get_image, create_image_from_multipart, create_image_from_json])
        .launch();
}