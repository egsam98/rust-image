extern crate image;

use crate::db::SqlitePool;
use crate::diesel::prelude::*;
use crate::schema::images::dsl::*;
use self::image::ImageFormat;
use self::image::imageops::FilterType;
use crate::models::{ImageForm, Image};
use reqwest::header::HeaderValue;
use std::ops::Try;


pub struct ImageService;

impl ImageService {
    pub fn find(_id: i32, pool: &SqlitePool) -> QueryResult<Image> {
        images.find(_id).first::<Image>(&pool.connection())
    }

    pub async fn upload_from_url(url: &str, pool: &SqlitePool) -> Result<i32, String> {
        let res = reqwest::get(url).await.map_err(|e| e.to_string())?;
        let _content_type: &HeaderValue = res.headers()
            .get("Content-Type")
            .into_result()
            .map_err(|_| "Content-Type not found".to_string())?;
        let content_type_str = _content_type.to_str().map_err(|e| e.to_string())?.to_string();
        let _bytes = res.bytes().await.map_err(|e| e.to_string())?.to_vec();

        let image_form = ImageForm {
            title: url.to_string(),
            content_type: content_type_str,
            bytes: _bytes,
        };
        Self::upload(&image_form, pool)
    }

    pub fn upload(image_form: &ImageForm, pool: &SqlitePool) -> Result<i32, String> {
        diesel::insert_into(images)
            .values(image_form)
            .execute(&pool.connection())
            .map_err(|e| e.to_string())?;

        let _id = pool.last_rowid("images");

        let preview = ImageService::generate_preview_image(image_form)?;

        diesel::update(images.filter(id.eq(&_id)))
            .set(preview_bytes.eq(&preview))
            .execute(&pool.connection())
            .map(|_| _id)
            .map_err(|e| e.to_string())
    }

    fn generate_preview_image(image_form: &ImageForm) -> Result<Vec<u8>, String> {
        let uploaded_image = image::load_from_memory(&image_form.bytes)
            .map_err(|e| e.to_string())?
            .resize(100, 100, FilterType::Lanczos3);
        let mut buff = Vec::new();
        uploaded_image.write_to(&mut buff, ImageFormat::Png)
            .map_err(|e| e.to_string())?;
        return Ok(buff);
    }
}