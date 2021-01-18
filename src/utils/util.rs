use chrono::Utc;

use crate::{
    utils::constants::{ONE_WEEK, NANOSECOND_THRESHOLD},
};

pub fn now_second() -> i64 {
    // nanosecond -> second
    Utc::now().timestamp_nanos() / NANOSECOND_THRESHOLD
}

pub fn one_week_second() -> i64 {
    now_second() + ONE_WEEK
}
