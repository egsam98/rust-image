use crate::services::image_service::ImageService;
use rocket::{State, Response};
use crate::utils::http::ToResponse;
use rocket::http::{ContentType, Status, RawStr};
use crate::db::SqlitePool;
use crate::utils::images_multipart::ImagesMultipart;
use std::collections::HashMap;
use rocket_contrib::json::Json;
use crate::utils::http::raw_response;
use std::str::FromStr;


#[get("/<id>?<preview>")]
pub fn get_image<'r>(id: &RawStr, preview: Option<bool>, pool: State<SqlitePool>) -> Response<'r> {
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
pub fn create_image_from_multipart<'r>(images_multipart: ImagesMultipart, pool: State<SqlitePool>) -> Response<'r> {
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
pub async fn create_image_from_json<'r>(data: Json<HashMap<&str, &str>>, pool: State<SqlitePool>) -> Response<'r> {
    let service_result = match data.get("url") {
        Some(url) => ImageService::upload_from_url(url, &pool.inner()).await,
        None => match data.get("base64") {
            Some(base64) => ImageService::upload_from_base64(base64, pool.inner()),
            None => return json_response!(400, {"error": "Must be any of json keys: [url, base64]"})
        }
    };
    match service_result {
        Ok(id) => json_response!({"id": id}),
        Err(e) => json_response!(400, {"error": e})
    }
}
