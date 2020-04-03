#![feature(proc_macro_hygiene, decl_macro)]
#![feature(try_trait)]

#[macro_use] extern crate log;
#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use crate::db::{SqlitePool};
use std::panic;

#[macro_use]mod utils;
mod db;
mod models;
mod services;
mod controllers;
mod schema;

const LOG_CONFIG_FILENAME: &str = "log4rs.yml";

fn configure_logger() {
    log4rs::init_file(LOG_CONFIG_FILENAME, Default::default()).unwrap();
    panic::set_hook(Box::new(|err| {
        error!("{:#?}", err);
    }));
}

fn main() {
    configure_logger();
    rocket::ignite()
        .manage(SqlitePool::new())
        .mount("/api/image", controllers::init_routes())
        .launch();
}