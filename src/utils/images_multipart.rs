use std::io::{Cursor, Read};
use multipart::server::Multipart;
use rocket::{
    data::{Data, FromData, Outcome, Transform, Transformed},
    http::Status,
    Request,
};
use crate::models::ImageForm;


macro_rules! try_opt_outcome {
    ($expr: expr, $err_msg: expr) => {
        match $expr {
            Some(ct) => ct,
            None => return Outcome::Failure((Status::BadRequest, $err_msg.to_string()))
        };
    };
}

macro_rules! try_res_outcome {
    ($expr: expr, $err_msg: expr) => {
        match $expr {
            Ok(ct) => ct,
            Err(_) => return Outcome::Failure((Status::BadRequest, $err_msg.to_string()))
        };
    };
}

#[derive(Debug)]
pub struct ImagesMultipart {
    pub images: Vec<ImageForm>,
}

impl<'a> FromData<'a> for ImagesMultipart {
    type Error = String;
    type Owned = Vec<u8>;
    type Borrowed = [u8];

    fn transform(_request: &Request, data: Data) -> Transform<Outcome<Self::Owned, Self::Error>> {
        let mut d = Vec::new();
        if let Err(e) = data.stream_to(&mut d) {
            let failure = Outcome::Failure((Status::InternalServerError, e.to_string()));
            return Transform::Owned(failure);
        }
        return Transform::Owned(Outcome::Success(d));
    }

    fn from_data(request: &Request, outcome: Transformed<'a, Self>) -> Outcome<Self, Self::Error> {
        let data = outcome.owned()?;
        let ct = try_opt_outcome!(request.headers().get_one("Content-Type"), "No Content-Type header");
        let boundary = &ct[(try_opt_outcome!(ct.find("boundary="), "No boundary") +
            "boundary=".len())..];

        let mut mp = Multipart::with_body(Cursor::new(data), boundary);

        let mut image_forms: Vec<ImageForm> = Vec::new();
        for entry in mp.read_entry() {
            if let Some(mut entry) = entry {
                match &*entry.headers.name {
                    "image" => {
                        let file_name = entry.headers.filename.unwrap_or_default();
                        let content_type = try_opt_outcome!(entry.headers.content_type, format!("No content type of '{}' filename", file_name));

                        let mut bytes = Vec::new();
                        try_res_outcome!(entry.data.read_to_end(&mut bytes),
                            format!("Not a file '{}'", file_name));

                        image_forms.push(ImageForm {
                            title: file_name,
                            content_type: content_type.to_string(),
                            bytes: bytes
                        });
                    }
                    other => {
                        let err_msg = format!("Unknown multipart key {}. Must be 'image'", other);
                        return try_opt_outcome!(None, err_msg)
                    },
                }
            }
        }

        let image_multipart = ImagesMultipart { images: image_forms };
        Outcome::Success(image_multipart)
    }
}