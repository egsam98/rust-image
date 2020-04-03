extern crate image;

use crate::db::SqlitePool;
use crate::diesel::prelude::*;
use crate::schema::images::dsl::*;
use self::image::ImageFormat;
use self::image::imageops::FilterType;
use crate::models::{ImageForm, Image};
use reqwest::header::HeaderValue;
use std::ops::Try;
use regex::Regex;


pub struct ImageService;

impl ImageService {
    pub fn find(_id: i32, pool: &SqlitePool) -> QueryResult<Image> {
        images.find(_id).first::<Image>(&pool.connection())
    }

    pub fn upload_from_base64(base64: &str, pool: &SqlitePool) -> Result<i32, String> {
        let err_str = "Incorrect base64 data. Must be 'data:image/<extension>:base64,<encoded>'";
        let re = Regex::new(r"data:(?P<type>.+);base64,(?P<encoded>.+)").unwrap();
        let captures = try_str!(re.captures(base64).into_result(), err_str);
        let encoded = try_str!(captures.name("encoded").into_result(), err_str).as_str();
        let bytes_decoded = try_str!(base64::decode(encoded));
        let _content_type = try_str!(captures.name("type").into_result(), err_str).as_str();
        let image_form = ImageForm {
            title: format!("{},base64", _content_type),
            content_type: _content_type.to_string(),
            bytes: bytes_decoded
        };

        Self::upload(&image_form, pool)
    }

    pub async fn upload_from_url(url: &str, pool: &SqlitePool) -> Result<i32, String> {
        let res = try_str!(reqwest::get(url).await);
        let _content_type: &HeaderValue = try_str!(res.headers().get("Content-Type").into_result(),
            "Content-Type not found");
        let content_type_str = try_str!(_content_type.to_str()).to_string();
        let _bytes = try_str!(res.bytes().await).to_vec();

        let image_form = ImageForm {
            title: url.to_string(),
            content_type: content_type_str,
            bytes: _bytes,
        };
        Self::upload(&image_form, pool)
    }

    pub fn upload(image_form: &ImageForm, pool: &SqlitePool) -> Result<i32, String> {
        if !Regex::new("^image/.+").unwrap().is_match(&image_form.content_type) {
           return Err(format!("Invalid image format '{}'. Must be 'image/<format>'",
                              image_form.content_type));
        }
        let preview = ImageService::generate_preview_image(image_form)?;
        try_str!(diesel::insert_into(images)
            .values((image_form, preview_bytes.eq(&preview)))
            .execute(&pool.connection()));
        Ok(pool.last_rowid("images"))
    }

    fn generate_preview_image(image_form: &ImageForm) -> Result<Vec<u8>, String> {
        let uploaded_image = try_str!(image::load_from_memory(&image_form.bytes))
            .resize(100, 100, FilterType::Lanczos3);
        let mut buff = Vec::new();
        let ext = image_form.content_type.replace("image/", "..");
        try_str!(uploaded_image.write_to(&mut buff, try_str!(ImageFormat::from_path(ext))));
        return Ok(buff);
    }
}