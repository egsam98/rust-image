use rocket::Route;


pub mod image_controller;

pub fn init_routes() -> Vec<Route> {
    routes!(
        image_controller::get_image,
        image_controller::create_image_from_multipart,
        image_controller::create_image_from_json
    )
}