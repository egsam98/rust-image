extern crate image;

use crate::db::SqlitePool;
use crate::diesel::prelude::*;
use crate::schema::images::dsl::*;
use self::image::ImageFormat;
use self::image::imageops::FilterType;
use crate::models::{ImageForm, Image};


pub struct ImageService;

impl ImageService {
    pub fn find(_id: i32, pool: &SqlitePool) -> QueryResult<Image> {
        images.find(_id).first::<Image>(&pool.connection())
    }

    pub fn upload(image_form: &ImageForm, pool: &SqlitePool) -> i32 {
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