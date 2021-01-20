use std::io::Error;

use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{NaiveDateTime, Utc};
use diesel::{prelude::*};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    config::db::{*, Connection},
    config::redis,
    schema::users::{self, dsl::*},
    utils::util,
};
// use diesel::result::Error;
use crate::utils::util::{now_second, one_week_second};
use crate::services::jwt::UserToken;

#[derive(Identifiable, Queryable, PartialEq, Clone, Serialize, Deserialize, Debug)]
#[table_name = "users"]
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

#[derive(Serialize, Deserialize, Default)]
pub struct LoginInfoDTO {
    pub username: String,
    pub email: String,
    pub login_session: String
}
#[derive(Insertable, Serialize, Deserialize, Default)]
#[table_name = "users"]
pub struct LoginInfoDTOTMP {
    pub id: i32,
    pub username: String,
    pub email: String,
}

impl User {
    pub fn signup(user: UserDTO, conn: &Connection) -> Option<LoginInfoDTO> {
        if Self::find_user_by_email(&user.email, conn).is_ok() {
            return None;
        }

        let hashed_pwd = hash(&user.password, DEFAULT_COST).unwrap();
        let user = UserDTO {
            password: hashed_pwd,
            ..user
        };

        diesel::insert_into(users).values(&user).execute(conn);

        let login_session_str = User::generate_login_session();
        Self::save_to_cache(&user.email, &login_session_str);

        Some(LoginInfoDTO {
            username: user.username,
            email: user.email,
            login_session: login_session_str
        })
    }

    pub fn login(login: LoginDTO, conn: &Connection) -> Option<LoginInfoDTO> {
        match Self::find_user_by_email(&login.email, conn) {
            Ok(user) => {
                let mut login_session_str = String::new();
                if verify(&login.password, &user.password).unwrap() {
                    login_session_str = User::generate_login_session();
                    Self::save_to_cache(&user.email, &login_session_str);
                }

                return Some(LoginInfoDTO {
                    username: user.username,
                    email: user.email,
                    login_session: login_session_str,
                });
            }
            Err(_) => None
        }
    }

    pub fn logout(e: &str) -> Result<(), String> {
        match redis::del_atomic_str(e) {
            Ok(_) => Ok(()),
            Err(err) => Err(err.detail().unwrap().to_string()),
        }
    }

    pub fn mul_insert(mul_users: Vec<UserDTO>, conn: &Connection) -> QueryResult<usize> {
        diesel::insert_into(users)
            .values(&mul_users)
            .execute(conn)
    }

    pub fn find_user_by_email(e: &str, conn: &Connection) -> QueryResult<User> {
        users.filter(email.eq(e)).get_result::<User>(conn)
    }

    pub fn generate_login_session() -> String {
        Uuid::new_v4().to_simple().to_string()
    }

    pub fn is_valid_login_session(user_token: &UserToken) -> bool {
        redis::get_atomic_str(&user_token.user).is_ok()
    }

    fn save_to_cache(login_email: &str, login_session: &str) {
        let ttl_second = now_second() + one_week_second();
        redis::set_atomic_str_with_ttl(login_email, &login_session, ttl_second as usize);
    }
}