use rocket_contrib::json::{Json, JsonValue};
use rocket::request::Form;
use rocket::http::RawStr;
use serde_json::Value;
use diesel::result::Error;

use crate::{
    config::db::Conn,
    models::task::*,
    constants::{message_constants, code_constants},
    utils::response::{ ResponseBody, ApiResponse },
    services::{jwt::UserToken, task_service},
};

#[get("/tasks", format = "application/json")]
pub fn index(
    token: Result<UserToken, ApiResponse>,
    conn: Conn
) -> ApiResponse {
    if let Err(err) = token {
        return err;
    }

    match task_service::index(token.unwrap(), &conn) {
        Ok(tasks) => ApiResponse::Ok().json(ResponseBody::new(code_constants::SUCCESS, message_constants::MESSAGE_OK, tasks)),
        Err(err) => err.response()
    }
}

#[post("/tasks", format = "application/json", data = "<new_task_dto>")]
pub fn create(
    token: Result<UserToken, ApiResponse>,
    new_task_dto: Json<NewTask>,
    conn: Conn
) -> ApiResponse {
    if let Err(err) = token {
        return err;
    }

    match task_service::add(token.unwrap(), new_task_dto.into_inner(), &conn) {
        Ok(_) => ApiResponse::Ok().json(ResponseBody::new(code_constants::SUCCESS, message_constants::MESSAGE_OK, "")),
        Err(err) => err.response()
    }
}
#[put("/tasks/<id>/finish", format = "application/json")]
pub fn finish(
    id: i32,
    token: Result<UserToken, ApiResponse>,
    conn: Conn
) -> ApiResponse {
    if let Err(err) = token {
        return err;
    }

    match task_service::finish(token.unwrap(), id, &conn) {
        Ok(_) => ApiResponse::Ok().json(ResponseBody::new(code_constants::SUCCESS, message_constants::MESSAGE_OK, "")),
        Err(err) => err.response()
    }
}
