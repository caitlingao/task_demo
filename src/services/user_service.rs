use rocket::http::Status;

use crate:: {
    config::db::*,
    models::user::*,
    constants::{message_constants, code_constants},
    utils::{error::ServiceError},
};

use super::jwt::UserToken;

#[derive(Serialize, Deserialize)]
pub struct TokenBodyResponse {
    pub token: String,
    pub token_type: String,
}

pub fn signup(user: UserDTO, conn: &Connection) -> Result<TokenBodyResponse, ServiceError> {
    match User::signup(user, &conn) {
        Some(logged_user) => {
            match serde_json::from_value(json!(UserToken::token_json(&logged_user))) {
                Ok(token_res) => {
                    Ok(token_res)
                }
                Err(e) => Err(ServiceError::new(Status::InternalServerError, code_constants::SIGN_UP_ERROR, message_constants::MESSAGE_SIGNUP_FAILED.to_string())),
            }
        }
        None => Err(ServiceError::new(Status::BadRequest, code_constants::SIGN_UP_ERROR, message_constants::MESSAGE_SIGNUP_FAILED.to_string())),
    }
}

pub fn login(login: LoginDTO, conn: &Connection) -> Result<TokenBodyResponse, ServiceError> {
    match User::login(login, &conn) {
        Some(logged_user) => {
            match serde_json::from_value(json!(UserToken::token_json(&logged_user))) {
                Ok(token_res) => {
                    if logged_user.login_session.is_empty() {
                        Err(ServiceError::new(Status::Unauthorized, code_constants::LOGIN_ERROR, message_constants::MESSAGE_LOGIN_FAILED.to_string()))
                    } else {
                        Ok(token_res)
                    }
                }
                Err(_) => Err(ServiceError::new(Status::InternalServerError, code_constants::INTERNAL_SERVER_ERROR, message_constants::MESSAGE_INTERNAL_SERVER_ERROR.to_string()))
            }
        }
        None => Err(ServiceError::new(Status::Unauthorized, code_constants::LOGIN_ERROR, message_constants::MESSAGE_USER_NOT_FOUND.to_string()))
    }
}

pub fn logout(user: &UserToken) -> Result<(), ServiceError> {
    match User::logout(&user.user) {
        Ok(_) => Ok(()),
        Err(err) => Err(ServiceError::new(Status::InternalServerError, code_constants::INTERNAL_SERVER_ERROR, err))
    }
}