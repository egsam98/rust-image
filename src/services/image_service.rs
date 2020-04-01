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
            .expect("Insert new image failed");

        let _id = pool.last_rowid("images");

        let preview = ImageService::generate_preview_image(&image_form.bytes);
        diesel::update(images.filter(id.eq(&_id)))
            .set(preview_bytes.eq(&preview))
            .execute(&pool.connection())
            .expect(&format!("Set preview bytes to image with ID '{}' failed", _id));

        return _id;
    }

    fn generate_preview_image(raw: &Vec<u8>) -> Vec<u8> {
        let uploaded_image = image::load_from_memory(raw)
            .expect("Image error")
            .resize(100, 100, FilterType::Lanczos3);
        let mut buff = Vec::new();
        uploaded_image.write_to(&mut buff, ImageFormat::Png)
            .expect("Error write to buff");
        return buff;
    }
}