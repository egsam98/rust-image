#![feature(proc_macro_hygiene, decl_macro)]
#![feature(try_trait)]

#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use crate::db::{SqlitePool};

#[macro_use]mod utils;
mod db;
mod models;
mod services;
mod controllers;
mod schema;

fn main() {
    rocket::ignite()
        .manage(SqlitePool::new())
        .mount("/api/image", controllers::init_routes())
        .launch();
}