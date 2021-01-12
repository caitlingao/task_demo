use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: i32,
    pub content: String,
    pub finished: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Task {
    pub fn new(content: &str, id: i32) -> Self {
        let now = Utc::now();
        Self {
            id,
            content: content.to_string(),
            finished: false,
            created_at: now.naive_utc(),
            updated_at: now.naive_utc(),
        }
    }
}