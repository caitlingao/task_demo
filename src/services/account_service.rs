use std::error::Error;
use std::fs::{self, OpenOptions};
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::{
    models::user::*,
    constants,
};

pub fn login(email: &str, password: &str) -> Result<(), Box<dyn Error>>{
    let path = Path::new(constants::USER_FILE);

    match get_metadata(path) {
        Ok(users) => {
            match users.iter().find(|user| user.email == email && user.password == password) {
                Some(user) => {
                    let cache_path = Path::new(constants::CACHE_FILE);
                    OpenOptions::new()
                        .read(true)
                        .write(true)
                        .create(true)
                        .open(&cache_path);

                    if let Ok(mut login_info) = get_login_metadata(cache_path) {
                        login_info.email = email.to_string();
                        login_info.id = user.clone().id;
                        login_info.username = user.clone().username;

                        write_to_file(&cache_path, &login_info);

                        println!("{}", constants::LOGIN_SUCCESS);
                    }
                },
                None => {
                    println!("{}", constants::LOGIN_FAILED)
                }
            }

        },
        Err(_) => {
            println!("{}", constants::GET_FILE_DATA_WRONG);
        }
    }

    Ok(())
}

pub fn logout() -> Result<(), Box<dyn Error>>{
    let path = Path::new(constants::CACHE_FILE);
    if fs::metadata(&path).is_err() {
        println!("{}", constants::LOGOUT_SUCCESS);
        return Ok(());
    }

    let login_info = LoginInfoDTO::new();

    write_to_file(path, &login_info);

    println!("{}", constants::LOGOUT_SUCCESS);

    Ok(())
}


fn get_metadata(path: &Path) -> Result<Vec<User>, Box<dyn Error>> {
    let string_data = fs::read_to_string(&path).expect(constants::UNABLE_TO_READ_FILE);
    let mut users: Vec<User> = vec![];
    if fs::metadata(&path).unwrap().len() != 0 {
        users = serde_json::from_str(&string_data)?;
    }

    Ok(users)
}

fn get_login_metadata(path: &Path) -> Result<LoginInfoDTO, Box<dyn Error>> {
    let string_data = fs::read_to_string(&path).expect(constants::UNABLE_TO_READ_FILE);
    let mut login_info = LoginInfoDTO::new();
    if fs::metadata(path).unwrap().len() != 0 {
        login_info = serde_json::from_str(&string_data)?;
    }

    Ok(login_info)
}

fn write_to_file(path: &Path, login_info: &LoginInfoDTO) -> Result<(), Box<dyn Error>>{
    let json: String = serde_json::to_string(login_info)?;
    fs::write(path, &json).expect(constants::UNABLE_WRITE_TO_FILE);

    Ok(())
}

pub fn get_current_user() -> Option<LoginInfoDTO> {
    let path = Path::new(constants::CACHE_FILE);
    if fs::metadata(constants::CACHE_FILE).is_err() {
        return None;
    }

    if let Ok(login_info) = get_login_metadata(path) {
        // 用户退出登录，恢复 LoginInfoDTO 结构为初始状态，此时 id 值为 0
        if login_info.id == 0 {
            return None;
        }
        return Some(login_info);
    }

    None
}