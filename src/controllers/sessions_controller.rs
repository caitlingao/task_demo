use rocket_contrib::json::{Json, JsonValue};

use crate::{
    config::db::Conn,
    models::user::LoginDTO,
    constants::{message_constants, code_constants},
    utils::{
        response::{ ResponseBody, ApiResponse, }
    },
    services::user_service,
    services::jwt::UserToken,
};

#[post("/sessions", format = "application/json", data = "<login_dto>")]
pub fn create(login_dto: Json<LoginDTO>, conn: Conn) -> ApiResponse {
    match user_service::login(login_dto.into_inner(), &conn) {
        Ok(token_response) => ApiResponse::Ok().json(ResponseBody::new(code_constants::SUCCESS, message_constants::MESSAGE_LOGIN_SUCCESS, token_response)),
        Err(err) => err.response()
    }
}

#[delete("/sessions", format = "application/json")]
pub fn destroy(token: Result<UserToken, ApiResponse>) -> ApiResponse {
    if let Err(err) = token {
        return err;
    }

    match user_service::logout(&token.unwrap()) {
        Ok(_) => ApiResponse::Ok().json(ResponseBody::new(code_constants::SUCCESS, message_constants::MESSAGE_LOGOUT_SUCCESS, "")),
        Err(err) => err.response(),
    }
}
