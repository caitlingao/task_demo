use std::fs::{self, OpenOptions};
use std::error::Error;
use std::path::Path;
use std::io::{self, Cursor, Write};

use chrono::{NaiveDateTime, Utc};
use itertools::Itertools;
use rocket::{
    http::{ContentType, Status},
    response::{Stream, status::Custom},
    Data,
};

use multipart::{
    mock::StdoutTee,
    server::{
        Multipart,
        save::{Entries, SaveResult::*},
    }
};

use crate::{
    models::{
        task::{Task, TaskDTO, NewTask},
    },
    constants::{message_constants, static_num_constants, code_constants},
    config::db::Connection,
    utils::{error::ServiceError},
};
use super::jwt::UserToken;

pub fn index(user: UserToken, conn: &Connection) -> Result<Vec<Task>, ServiceError> {
    match Task::index(&user.user, conn) {
        Ok(tasks) => Ok(tasks),
        Err(err) => Err(ServiceError::new(Status::InternalServerError, code_constants::INTERNAL_SERVER_ERROR, err.to_string()))
    }
}

pub fn add(user: UserToken, task_dto: NewTask, conn: &Connection) -> Result<(), ServiceError> {
    match Task::insert(&user.user, task_dto, conn) {
        Ok(_) => Ok(()),
        Err(err) => Err(ServiceError::new(Status::InternalServerError, code_constants::INTERNAL_SERVER_ERROR, err.to_string()))
    }
}

pub fn finish(user: UserToken, id: i32, conn: &Connection) -> Result<(), ServiceError> {
    match Task::finish(&user.user, id, conn) {
        Ok(s) => Ok(()),
        Err(err) => Err(ServiceError::new(Status::InternalServerError, code_constants::INTERNAL_SERVER_ERROR, err.to_string()))
    }
}

pub fn export(user: UserToken, conn: &Connection) -> Result<(), ServiceError> {
    match Task::index(&user.user, conn) {
        Ok(tasks) => {
            match export_tasks(tasks) {
                Ok(_) => Ok(()),
                Err(err) => Err(ServiceError::new(Status::InternalServerError, code_constants::INTERNAL_SERVER_ERROR, err.to_string()))
            }
        },
        Err(err) => Err(ServiceError::new(Status::InternalServerError, code_constants::INTERNAL_SERVER_ERROR, err.to_string()))
    }
}

fn export_tasks(tasks: Vec<Task>) -> Result<(), Box<dyn Error>>{
    if fs::metadata(static_num_constants::TMP_DIR).is_err() {
        fs::create_dir(message_constants::TMP_DIR);
    }
    if fs::metadata(static_num_constants::DOWNLOAD_DIR).is_err() {
        fs::create_dir(message_constants::DOWNLOAD_DIR);
    }

    let download_file_path = &format!("{download_dir}/{file_name}",
                                      download_dir = static_num_constants::DOWNLOAD_DIR,
                                      file_name = static_num_constants::DOWNLOAD_FILE);
    let download_path = Path::new(download_file_path);
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&download_path);

    write_to_file(&download_path, &tasks);

    Ok(())
}

fn write_to_file(path: &Path, tasks: &Vec<Task>) -> Result<(), Box<dyn Error>>{
    let json: String = serde_json::to_string(tasks)?;
    fs::write(path, &json).expect(message_constants::UNABLE_WRITE_TO_FILE);

    Ok(())
}
