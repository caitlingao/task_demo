use rocket::Outcome;
use rocket::http::{ContentType, Status};
use rocket::request::{self, Request, FromRequest};
use rocket::response::{self, Responder, Response};
use rocket_contrib::json::{Json, JsonValue};
use serde_json::Value;

#[derive(Debug)]
pub struct ApiResponse {
    pub json: Value,
    pub status: Status,
}

impl<'r> Responder<'r> for ApiResponse {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        let mut body = JsonValue::from(json!(self.json));
        Response::build_from(body.respond_to(req).unwrap())
            .status(self.status)
            .header(ContentType::JSON)
            .ok()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseBody<T> {
    pub code: i32,
    pub message: String,
    pub data: T,
}

impl<T> ResponseBody<T> {
    pub fn new(code: i32, message: &str, data: T) -> ResponseBody<T> {
        ResponseBody {
            code,
            message: message.to_string(),
            data,
        }
    }
}
