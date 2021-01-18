use rocket::http::Status;

use crate:: {
    config::db::*,
    models::user::{User, UserDTO, LoginInfoDTO},
    utils::{error::ServiceError, constant_code::*, constants::*},
};

use super::user_token::UserToken;

#[derive(Serialize, Deserialize)]
pub struct TokenBodyResponse {
    pub token: String,
    pub token_type: String,
}

pub fn signup(user: UserDTO, conn: &Connection) -> Result<TokenBodyResponse, ServiceError> {
    match User::signup(user, &conn) {
        Some(login_info_dto) => {
            match serde_json::from_value(json!(UserToken::token_json(login_info_dto))) {
                Ok(token_res) => {
                    Ok(token_res)
                }
                Err(e) => Err(ServiceError::new(Status::InternalServerError, SIGN_UP_ERROR, MESSAGE_SIGNUP_FAILED.to_string())),
            }
        }
        None => Err(ServiceError::new(Status::BadRequest, SIGN_UP_ERROR, MESSAGE_SIGNUP_FAILED.to_string())),
    }
}