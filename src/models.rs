use crate::schema::images;


#[derive(Queryable)]
pub struct Image {
    pub id: i32,
    pub title: String,
    pub content_type: String,
    pub bytes: Vec<u8>,
    pub preview_bytes: Option<Vec<u8>>
}

#[derive(Insertable, Debug)]
#[table_name = "images"]
pub struct ImageForm {
    pub title: String,
    pub content_type: String,
    pub bytes: Vec<u8>
}