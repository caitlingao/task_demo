#![feature(plugin, decl_macro, proc_macro_hygiene)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate redis;
extern crate r2d2_redis;

extern crate dotenv;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
extern crate rocket_cors;
extern crate multipart;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate log;
extern crate env_logger;

use dotenv::dotenv;

mod cli;
mod config;
mod constants;
mod controllers;
mod models;
mod schema;
mod services;
mod utils;

fn main() {
    dotenv().ok();

    config::rocket::rocket().launch();
}
