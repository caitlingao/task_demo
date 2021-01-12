use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct LoginInfoDTO {
    pub id: i32,
    pub username: String,
    pub email: String,
}

impl LoginInfoDTO {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}