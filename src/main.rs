#![feature(proc_macro_hygiene, decl_macro)]
#![feature(try_trait)]

#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use rocket_contrib::serve::StaticFiles;
use crate::db::{SqlitePool};
use rocket_contrib::templates::Template;
use std::collections::HashMap;
use rocket::Response;
use crate::utils::http::ToResponse;

#[macro_use]mod utils;
mod db;
mod models;
mod services;
mod controllers;
mod schema;

const LOG_CONFIG_FILENAME: &str = "log4rs.yml";
const STATIC_PATH_PREFIX: &str = "static";

#[get("/")]
fn docs() -> Template {
    Template::render("index", HashMap::<String, String>::new())
}

#[catch(404)]
fn handler_404<'r>() -> Response<'r> {
    json_response!(404, {"error": "No matching routes to handle this request"})
}

fn main() {
    log4rs::init_file(LOG_CONFIG_FILENAME, Default::default()).unwrap();
    rocket::ignite()
        .manage(SqlitePool::new())
        .mount("/", routes!(docs))
        .mount("/docs", routes!(docs))
        .mount("/public", StaticFiles::from(STATIC_PATH_PREFIX))
        .mount("/api/image", controllers::init_routes())
        .register(catchers!(handler_404))
        .attach(Template::fairing())
        .launch();
}