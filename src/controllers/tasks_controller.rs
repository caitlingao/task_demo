use rocket_contrib::json::{Json, JsonValue};
use serde_json::Value;
use diesel::result::Error;

use crate::{
    config::db::*,
    models::task::*,
    utils::response::ResponseBody,
};

#[get("/tasks", format = "application/json")]
pub fn index(conn: Conn) -> Result<Json<ResponseBody<Vec<Task>>>, Error> {
    match Task::find_all(1, &conn) {
       Ok(tasks) => {
           Ok(Json(ResponseBody::new(0, "success", tasks)))
       }
        Err(err) => {
            Err(err)
        }
    }
}
