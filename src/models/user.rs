use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{NaiveDateTime, Utc};
use diesel::{prelude::*, pg::PgConnection};
use serde::{Deserialize, Serialize};

use crate::{
    constants,
    schema::users::{self, dsl::*},
    config::db::Connection,
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
    pub fn signup(user: UserDTO, conn: &Connection) -> Option<UserDTO> {
        if Self::find_user_by_email(&user.email, conn).is_ok() {
            return None;
        }

        let hashed_pwd = hash(&user.password, DEFAULT_COST).unwrap();
        let user = UserDTO {
            password: hashed_pwd,
            ..user
        };
        diesel::insert_into(users).values(&user).execute(conn);
        Some(UserDTO{..user})
    }

    pub fn login(login: LoginDTO, conn: &PgConnection) -> Option<LoginInfoDTO> {
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

    pub fn mul_insert(mul_users: Vec<UserDTO>, conn: &PgConnection) -> QueryResult<usize> {
        diesel::insert_into(users)
            .values(&mul_users)
            .execute(conn)
    }

    pub fn find_user_by_email(e: &str, conn: &Connection) -> QueryResult<User> {
        users.filter(email.eq(e)).get_result::<User>(conn)
    }
}