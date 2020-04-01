use std::io::{Cursor, Read};
use multipart::server::Multipart;
use rocket::{
    data::{Data, FromData, Outcome, Transform, Transformed},
    Request,
};
use crate::models::ImageForm;

#[derive(Debug)]
pub struct ImagesMultipart {
    pub images: Vec<ImageForm>,
}

impl<'a> FromData<'a> for ImagesMultipart {
    type Error = ();
    type Owned = Vec<u8>;
    type Borrowed = [u8];


    fn transform(_request: &Request, data: Data) -> Transform<Outcome<Self::Owned, Self::Error>> {
        let mut d = Vec::new();
        data.stream_to(&mut d).expect("Unable to read");
        return Transform::Owned(Outcome::Success(d));
    }

    fn from_data(request: &Request, outcome: Transformed<'a, Self>) -> Outcome<Self, Self::Error> {
        let data = outcome.owned().expect("No multipart data");
        let ct = request.headers().get_one("Content-Type").expect("No content-type");
        let boundary = &ct[(ct.find("boundary=").expect("No boundary") + "boundary=".len())..];

        let mut mp = Multipart::with_body(Cursor::new(data), boundary);

        let mut image_forms: Vec<ImageForm> = Vec::new();
        mp.foreach_entry(|mut entry|
            match &*entry.headers.name {
                "image" => {
                    let file_name = entry.headers.filename.unwrap_or_default();
                    let content_type = entry.headers.content_type
                        .expect(&format!("No content type of '{}'", file_name)).to_string();
                    let mut bytes = Vec::new();
                    entry.data.read_to_end(&mut bytes).expect(&format!("Not a file '{}'", file_name));
                    image_forms.push(ImageForm{
                        title: file_name,
                        content_type: content_type,
                        bytes: bytes
                    });
                }
                other => panic!("No known key {}", other),
            }).expect("Unable to iterate multipart data");

        let image_multipart = ImagesMultipart { images: image_forms };
        Outcome::Success(image_multipart)
    }
}