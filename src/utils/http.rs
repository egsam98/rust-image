use rocket::Response;
use rocket::http::{ContentType, Status};
use std::io::Cursor;
use rocket_contrib::json::JsonValue;


pub trait ToResponse<'r> {
    fn to_response(&self, status: Option<Status>) -> Response<'r>;
}

impl <'r> ToResponse<'r> for JsonValue {
    fn to_response(&self, status: Option<Status>) -> Response<'r> {
        Response::build()
            .status(status.unwrap_or(Status::Ok))
            .header(ContentType::JSON)
            .sized_body(Cursor::new(self.to_string()))
            .finalize()
    }
}

pub fn raw_response<'r>(bytes: Vec<u8>, content_type: ContentType) -> Response<'r> {
    Response::build()
        .header(content_type)
        .sized_body(Cursor::new(bytes))
        .finalize()
}