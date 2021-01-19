use serde_json::Value;
use jsonwebtoken::{
    EncodingKey,
    DecodingKey,
    Header,
    Validation,
    Algorithm,
    TokenData,
    errors::Result,
};
use rocket::{
    http::Status,
    outcome::Outcome,
    request::{self, FromRequest, Request},
    response::status,
};
use rocket_contrib::json::{Json, JsonValue};
use serde::Serialize;

use crate::{
    models::user::{User, LoginInfoDTO},
    utils::{util, response::ApiResponse},
    constants::message_constants,
};
use crate::utils::util::{now_second, one_week_second};

pub static KEY: [u8; 16] = *include_bytes!("../secret.key");
// static ONE_WEEK: i64 = 60 * 60 * 24 * 7; // in seconds

#[derive(Debug, Serialize, Deserialize, FromForm)]
pub struct UserToken {
    // issued at
    pub iat: i64,
    // expiration
    pub exp: i64,
    // data
    pub user: String,
    pub login_session: String,
}

impl<'a, 'r> FromRequest<'a, 'r> for UserToken {
    // type Error = status::Custom<Json<ApiResponse>>;
    // type Error = JsonValue;
    type Error = ApiResponse;
    fn from_request(
        request: &'a Request<'r>,
    ) -> request::Outcome<Self, Self::Error> {
        // let conn = request.guard::<DbConn>().unwrap();
        if let Some(authen_header) = request.headers().get_one("Authorization") {
            let authen_str = authen_header.to_string();
            if authen_str.starts_with("Bearer") {
                let token = authen_str[6..authen_str.len()].trim();
                if let Ok(token_data) = UserToken::decode_token(token.to_string()) {
                    if UserToken::verify_token(&token_data) {
                        return Outcome::Success(token_data.claims);
                    }
                }
            }
        }

        // Outcome::Failure((Status::Unauthorized, JsonValue::from(json!({ "error": message_constants::MESSAGE_INVALID_TOKEN }))))
        Outcome::Failure((
            Status::Unauthorized,
            ApiResponse {
                status: Status::Unauthorized,
                json: json!({
                    "code": 200,
                    "message": message_constants::MESSAGE_INVALID_TOKEN,
                    "data": String::new(),
                })
            }
        ))
    }
}

impl UserToken {
    pub fn generate_token(login: &LoginInfoDTO) -> String {
        let payload = UserToken {
            iat: now_second(),
            exp: one_week_second(),
            user: login.email.clone(),
            login_session: login.login_session.clone(),
        };

        let header = Header::new(Algorithm::HS256);
        jsonwebtoken::encode(&header, &payload, &EncodingKey::from_secret(&KEY)).unwrap()
    }

    pub fn token_json(login: &LoginInfoDTO) -> Value {
        json!({
            "token": Self::generate_token(login),
            "token_type": "bearer",
        })
    }

    pub fn decode_token(token: String) -> Result<TokenData<UserToken>> {
        jsonwebtoken::decode::<UserToken>(&token, &DecodingKey::from_secret(&KEY), &Validation::default())
    }

    pub fn verify_token(token_data: &TokenData<UserToken>) -> bool {
        User::is_valid_login_session(&token_data.claims)
    }
}