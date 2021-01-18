use rocket::http::Status;

use super::response::{ResponseBody, ApiResponse};

#[derive(Debug)]
pub struct ServiceError {
    pub status: Status,
    pub body: ResponseBody<String>,
}

impl ServiceError {
    pub fn new(status: Status, code: i32, message: String) -> ServiceError {
        ServiceError {
            status,
            body: ResponseBody {
                code,
                message,
                data: String::new(),
            }
        }
    }

    pub fn response(&self) -> ApiResponse {
        ApiResponse {
            json: json!(self.body),
            status: self.status
        }
    }
}