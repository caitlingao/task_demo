use rocket_contrib::json::{Json, JsonValue};
// use serde_json::Value;
use diesel::result::Error;
use rocket::http::Status;

use crate::{
    config::db::*,
    models::user::*,
    utils::response::{ResponseBody, ApiResponse},
};

#[post("/users", format = "application/json", data = "<user_dto>")]
pub fn create(user_dto: Json<UserDTO>, conn: Conn) -> ApiResponse {
    match User::signup(user_dto.into_inner(), &conn) {
        Some(user) => ApiResponse {
            json: json!(ResponseBody::new(0, "success", user)),
            status: Status::Ok
        },
        None => ApiResponse {
            json: json!(ResponseBody::new(10000, "success", "")),
            status: Status::BadRequest
        }
    }
}
