use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{NaiveDateTime, Utc};
use diesel::{prelude::*};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    config::db::{*, Connection},
    schema::users::{self, dsl::*},
    utils::util,
};
use diesel::result::Error;
use crate::utils::util::{now_second, one_week_second};

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

        let login_session_str = User::generate_login_session();
        diesel::insert_into(users).values(&user).execute(conn);
        let ttl_second = now_second() + one_week_second();
        set_atomic_str_with_ttl(&user.email, &login_session_str, ttl_second as usize);

        Some(LoginInfoDTO {
            username: user.username,
            email: user.email,
            login_session: login_session_str
        })
    }

    pub fn login(login: LoginDTO, conn: &Connection) -> Option<LoginInfoDTO> {
        if let Ok(user) = users
            .filter(email.eq(login.email))
            .get_result::<User>(conn)
        {
           if verify(&login.password, &user.password).unwrap() {
               return Some(LoginInfoDTO {
                   username: user.username,
                   email: user.email,
                   login_session: User::generate_login_session(),
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

    pub fn find_user_by_email(e: &str, conn: &Connection) -> QueryResult<User> {
        users.filter(email.eq(e)).get_result::<User>(conn)
    }

    pub fn generate_login_session() -> String {
        Uuid::new_v4().to_simple().to_string()
    }
}