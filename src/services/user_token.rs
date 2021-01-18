use serde_json::Value;
use chrono::Utc;
use jsonwebtoken::{
    EncodingKey,
    Header,
    Algorithm,
};

use crate::{
    models::user::LoginInfoDTO,
    utils::util,
};
use crate::utils::util::{now_second, one_week_second};

pub static KEY: [u8; 16] = *include_bytes!("../secret.key");
// static ONE_WEEK: i64 = 60 * 60 * 24 * 7; // in seconds

#[derive(Serialize, Deserialize)]
pub struct UserToken {
    // issued at
    pub iat: i64,
    // expiration
    pub exp: i64,
    // data
    pub user: String,
    pub login_session: String,
}

impl UserToken {
    pub fn generate_token(login: LoginInfoDTO) -> String {
        let payload = UserToken {
            iat: now_second(),
            exp: one_week_second(),
            user: login.email,
            login_session: login.login_session,
        };

        let header = Header::new(Algorithm::HS256);
        jsonwebtoken::encode(&header, &payload, &EncodingKey::from_secret(&KEY)).unwrap()
    }

    pub fn token_json(login: LoginInfoDTO) -> Value {
        json!({
            "token": Self::generate_token(login),
            "token_type": "bearer",
        })
    }
}