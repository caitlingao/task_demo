use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    config::db::Connection,
    schema::users::{self, dsl::*},
};

#[derive(Queryable, Clone, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize, Debug, Clone)]
#[table_name = "users"]
pub struct UserDTO {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginDTO {
    pub email: String,
    pub password: String,
}

#[derive(Insertable, Serialize, Deserialize, Default)]
#[table_name = "users"]
pub struct LoginInfoDTO {
    pub id: i32,
    pub username: String,
    pub email: String,
}

impl User {
    pub fn login(login: LoginDTO, conn: &Connection) -> Option<LoginInfoDTO> {
        if let Ok(user) = users
            .filter(email.eq(login.email))
            .get_result::<User>(conn)
        {
           if verify(&login.password, &user.password).unwrap() {
               return Some(LoginInfoDTO {
                   id: user.id,
                   username: user.username,
                   email: user.email,
               });
           }
        } else {
            return None;
        }
        None
    }

    pub fn mul_insert(mul_users: Vec<UserDTO>, conn: &Connection) -> QueryResult<usize> {
        diesel::insert_into(users)
            .values(&mul_users)
            .execute(conn)
    }
}