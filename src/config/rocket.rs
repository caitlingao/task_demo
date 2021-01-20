use rocket::Rocket;
use rocket::fairing::AdHoc;
use rocket::http::Method;
use rocket_cors::{Cors, AllowedHeaders, AllowedMethods, AllowedOrigins};

use super::db;
use crate::controllers::*;
use crate::services::jwt;

pub fn rocket() -> Rocket {
    let pool = db::pool();
    rocket::ignite()
        .manage(pool)
        .attach(AdHoc::on_attach("Database Migrations", db::run_db_migrations))
        .attach(get_cors())
        .mount(
            "/api/v1/",
            routes![
                tasks_controller::index,
                tasks_controller::create,
                tasks_controller::finish
            ],
        )
        .mount(
            "/api/v1/",
            routes![users_controller::create],
        )
        .mount(
            "/api/v1/",
            routes![sessions_controller::create, sessions_controller::destroy],
        )
}

fn get_cors() -> Cors {
    rocket_cors::CorsOptions {
        allowed_origins: AllowedOrigins::all(),
        allowed_methods: vec![Method::Get, Method::Post, Method::Put, Method::Patch, Method::Delete].into_iter().map(From::from).collect(),
        // allowed_headers: AllowedHeaders::all(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }.to_cors().expect("cors config error")
}