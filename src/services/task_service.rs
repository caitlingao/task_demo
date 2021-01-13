use std::fs::{self, OpenOptions};
use std::error::Error;
use std::path::Path;

use chrono::{NaiveDateTime, Utc};
use itertools::Itertools;

use crate::{
    config::db::Connection,
    constants,
    models::{
        task::{Task, TaskDTO},
    },
    services::account_service,
};

pub fn get_tasks(conn: &Connection) {
    let user_id = account_service::get_current_user().unwrap().id;
    match Task::find_all(user_id, conn) {
        Ok(tasks) => {
            let total = tasks.len();
            let mut finished_count = 0;
            for task in tasks.iter() {
                let mut message = format!("{id}. {content}", id = task.id, content = task.content);
                if task.finished {
                    finished_count += 1;
                    message = format!("{id}. [Done]{content}", id = task.id, content = task.content);
                }
                println!("{}", message);
            }
            println!();
            let total_word = get_singular_plural(total, "item".to_string());
            let finished_count_word = get_singular_plural(finished_count, "item".to_string());
            println!("Total: {total} {total_word}, {finished_count} {finished_count_word} done",
                     total = total,
                     total_word = total_word,
                     finished_count = finished_count,
                     finished_count_word = finished_count_word
            );
        },
        Err(err) => {
            println!("find tasks err: {:?}", err);
        }
    }
}

pub fn get_unfinished_tasks(conn: &Connection) {
    let user_id = account_service::get_current_user().unwrap().id;
    match Task::find_unfinished(user_id, conn) {
        Ok(tasks) => {
            let total = tasks.len();
            for task in tasks.iter() {
                println!("{id}. {content}", id = task.id, content = task.content);
            }
            println!();
            let word = get_singular_plural(total, "item".to_string());
            println!("Total: {total} {word}", total = total, word = word);
        },
        Err(err) => {
            println!("find tasks err: {:?}", err);
        }
    }
}

pub fn add_task(content: &str, conn: &Connection) {
    let user_id = account_service::get_current_user().unwrap().id;
    let task = TaskDTO {
        user_id,
        content: content.to_string(),
        finished: false,
    };

    match Task::insert(task, conn) {
        Ok(id) => {
            println!("{id}. {content}", id = id, content = content);
        },
        Err(err) => {
            println!("insert task error. {:?}", err);
        }
    }
}

pub fn finish_task(id: i32, conn: &Connection) {
    let user_id = account_service::get_current_user().unwrap().id;
    match Task::finish_task(id, user_id, conn) {
        Ok(_) => {
            println!("Item {} done.", id);
        },
        Err(err) => {
            println!("update task to finished error. {:?}", err);
        }
    }
}

pub fn export_tasks(file_name: &str, conn: &Connection) -> Result<(), Box<dyn Error>>{
    if fs::metadata(constants::DOWNLOAD_DIR).is_err() {
        fs::create_dir(constants::DOWNLOAD_DIR);
    }

    let download_file_path = &format!("{download_dir}/{file_name}.json",
                                      download_dir = constants::DOWNLOAD_DIR,
                                      file_name = file_name);
    let download_path = Path::new(download_file_path);
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&download_path);

    let user_id = account_service::get_current_user().unwrap().id;
    match Task::find_all(user_id, conn) {
        Ok(tasks) => {
            write_to_file(&download_path, &tasks);
            // let json: String = serde_json::to_string(&tasks)?;
            // fs::write(&download_path, &json).expect(constants::UNABLE_WRITE_TO_FILE);

            let total = tasks.len();
            let word = get_singular_plural(total, "item".to_string());
            println!("Export success. {total} {word} exported.", total = total, word = word);
        }
        Err(err) => {
            println!("find all task error {:?}", err);
        }
    }
    Ok(())
}

pub fn import_tasks(file_name: &str, conn: &Connection) {
    if !file_name.ends_with(constants::IMPORT_FILE_SUFFIX) {
        println!("{}",constants::ASK_FOR_JSON_FILE);
        return;
    }
    if fs::metadata(file_name).is_err() {
        println!("{}",constants::FILE_NOT_EXIST);
        return;
    }

    let import_file_path = Path::new(file_name);
    match get_metadata(import_file_path) {
        Ok(waiting_tasks) => {
            let mut tasks: Vec<TaskDTO> = vec![];
            // 去重
            let purified_tasks = waiting_tasks
                .iter()
                .unique_by(|task| &task.user_id)
                .unique_by(|task| &task.content)
                .unique_by(|task| &task.finished)
                .collect::<Vec<_>>();
            let mut success_count = purified_tasks.len();
            for task in purified_tasks.iter() {
                tasks.push(task.clone().clone());

            }
            Task::mul_insert(tasks, conn);

            println!("Import tasks success, success {}.", success_count);
        }
        Err(e) => {
            println!("err: {:?}", e);
            println!("{}",constants::GET_FILE_DATA_WRONG);
        }

    }
}

pub fn init_tasks(conn: &Connection) {
    import_tasks(constants::TASKS_FILE, conn);
}

fn get_metadata(path: &Path) -> Result<Vec<TaskDTO>, Box<dyn Error>> {
    let mut tasks: Vec<TaskDTO> = vec![];
    if fs::metadata(&path).is_err() {
        return Ok(tasks);
    }

    let string_data = fs::read_to_string(&path)?;
    if fs::metadata(&path).unwrap().len() != 0 {
        tasks = serde_json::from_str(&string_data)?;
    }

    Ok(tasks)
}

fn write_to_file(path: &Path, tasks: &Vec<Task>) -> Result<(), Box<dyn Error>>{
    let json: String = serde_json::to_string(tasks)?;
    fs::write(path, &json).expect(constants::UNABLE_WRITE_TO_FILE);

    Ok(())
}

fn get_singular_plural(count: usize, word: String) -> String {
    if count > constants::SINGULAR_PLURAL_THRESHOLD as usize {
        format!("{}s", word)
    } else {
        format!("{}", word)
    }
}
