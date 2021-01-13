use std::error::Error;
use std::fs::{self, OpenOptions};
use std::io::{Result};
use std::path::Path;

use bcrypt::{hash, DEFAULT_COST};
use itertools::Itertools;

use crate::{
    config::db::Connection,
    constants,
    models::user::*,
};

pub fn login(email: &str, password: &str, conn: &Connection) -> Result<()> {
    let login_dto = LoginDTO {
        email: email.to_string(),
        password: password.to_string(),
    };
    match User::login(login_dto, conn) {
        Some(login_info_dto) => {
            let cache_path = Path::new(constants::CACHE_FILE);
            OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(&cache_path);

            if let Ok(mut login_info) = get_login_metadata(&cache_path) {
                login_info.email = email.to_string();
                login_info.id = login_info_dto.id;
                login_info.username = login_info_dto.username;

                write_to_file(&cache_path, &login_info);

                println!("{}", constants::LOGIN_SUCCESS);
            }
        },
        None => {
            println!("{}",constants::LOGIN_FAILED);
            return Ok(());
        }
    }
    Ok(())
}

pub fn logout() -> Result<()>{
    let path = Path::new(constants::CACHE_FILE);
    if fs::metadata(&path).is_err() {
        println!("{}",constants::LOGOUT_SUCCESS);
        return Ok(());
    }

    let login_info = LoginInfoDTO {
        ..Default::default()
    };
    write_to_file(path, &login_info);

    println!("{}",constants::LOGOUT_SUCCESS);

    Ok(())
}

pub fn import_users(conn: &Connection) {
    let import_file_path = Path::new(constants::USER_FILE);
    match get_metadata(import_file_path) {
        Ok(waiting_users) => {
            let mut users: Vec<UserDTO> = vec![];
            let mut success_count = waiting_users.len();
            for waiting_user in waiting_users.iter() {
                let hashed_pwd = hash(&waiting_user.password, DEFAULT_COST).unwrap();
                let new_user = UserDTO {
                    password: hashed_pwd,
                    ..waiting_user.clone()
                };
                users.push(new_user);

                println!("--- handling user with email: {:?}", waiting_user.email);
            }
            User::mul_insert(users, conn);

            println!("Import users success, success {}.", success_count);
        }
        Err(_) => {
            println!("{}",constants::GET_FILE_DATA_WRONG);
        }

    }
}

pub fn get_current_user() -> Option<LoginInfoDTO> {
    let cache_path = Path::new(constants::CACHE_FILE);
    if fs::metadata(constants::CACHE_FILE).is_err() {
        return None;
    }
    let string_data = fs::read_to_string(&cache_path).expect(constants::UNABLE_TO_READ_FILE);
    let mut login_info = LoginInfoDTO {
        ..Default::default()
    };
    if fs::metadata(&cache_path).unwrap().len() != 0 {
        login_info = serde_json::from_str(&string_data).expect(constants::UNABLE_TO_READ_FILE);
    }

    // 用户退出登录，恢复 LoginInfo 结构为初始状态，此时 id 值为 0
    if login_info.id == 0 {
        return None;
    }

    Some(login_info)
}

fn get_metadata(path: &Path) -> Result<Vec<UserDTO>> {
    let mut users: Vec<UserDTO> = vec![];
    if fs::metadata(&path).is_err() {
        return Ok(users);
    }

    let string_data = fs::read_to_string(&path)?;
    if fs::metadata(&path).unwrap().len() != 0 {
        users = serde_json::from_str(&string_data)?;
    }

    // 对原数据库中user数据进行去重；
    if users.len() > 1 {
        users = users
            .iter()
            .unique_by(|user| &user.email)
            .cloned()
            .collect::<Vec<_>>();
    }

    Ok(users)
}

fn get_login_metadata(path: &Path) -> Result<LoginInfoDTO> {
    let string_data = fs::read_to_string(&path).expect(constants::UNABLE_TO_READ_FILE);
    let mut login_info = LoginInfoDTO {
        ..Default::default()
    };
    if fs::metadata(&path).unwrap().len() != 0 {
        login_info = serde_json::from_str(&string_data)?;
    }

    Ok(login_info)
}

fn write_to_file(path: &Path, login_info: &LoginInfoDTO) -> Result<()>{
    let json: String = serde_json::to_string(login_info)?;
    fs::write(path, &json).expect(constants::UNABLE_WRITE_TO_FILE);

    Ok(())
}
