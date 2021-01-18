use rocket_contrib::json::{Json, JsonValue};

use crate::{
    config::db::*,
    models::user::*,
    utils::{
        constant_code::*,
        constants::*,
        response::{
            ResponseBody,
            ApiResponse,
        }
    },
    services::user_service,
};

#[post("/users", format = "application/json", data = "<user_dto>")]
pub fn create(user_dto: Json<UserDTO>, conn: Conn) -> ApiResponse {
    match user_service::signup(user_dto.into_inner(), &conn) {
        Ok(token_response) => ApiResponse::Ok().json(ResponseBody::new(SUCCESS, MESSAGE_SIGNUP_SUCCESS, token_response)),
        Err(err) => err.response()
    }
}
