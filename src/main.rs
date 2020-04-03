#![feature(proc_macro_hygiene, decl_macro)]
#![feature(try_trait)]

#[macro_use] extern crate log;
#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use rocket_contrib::serve::StaticFiles;
use crate::db::{SqlitePool};
use std::panic;
use rocket_contrib::templates::Template;
use std::collections::HashMap;

#[macro_use]mod utils;
mod db;
mod models;
mod services;
mod controllers;
mod schema;

const LOG_CONFIG_FILENAME: &str = "log4rs.yml";
const STATIC_PATH_PREFIX: &str = "static";

fn configure_logger() {
    log4rs::init_file(LOG_CONFIG_FILENAME, Default::default()).unwrap();
    panic::set_hook(Box::new(|err| {
        error!("{:#?}", err);
    }));
}

#[get("/")]
fn docs() -> Template {
    Template::render("index", HashMap::<String, String>::new())
}

fn main() {
    configure_logger();
    rocket::ignite()
        .manage(SqlitePool::new())
        .mount("/", routes!(docs))
        .mount("/docs", routes!(docs))
        .mount("/public", StaticFiles::from(STATIC_PATH_PREFIX))
        .mount("/api/image", controllers::init_routes())
        .attach(Template::fairing())
        .launch();
}