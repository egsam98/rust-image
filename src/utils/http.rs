use rocket::Response;
use rocket::http::{ContentType, Status};
use std::io::Cursor;


pub fn error_json_response<'r>(text: &str, status: Status) -> Response<'r> {
    let text_json = format!("{{\"error\": \"{}\"}}", text);
    Response::build()
        .status(status)
        .header(ContentType::new("application", "json"))
        .sized_body(Cursor::new(text_json))
        .finalize()
}

pub fn raw_response<'r>(bytes: Vec<u8>, content_type: ContentType) -> Response<'r> {
    Response::build()
        .header(content_type)
        .sized_body(Cursor::new(bytes))
        .finalize()
}