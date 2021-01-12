use std::fs::{self, OpenOptions};
use std::io::{Result};
use std::path::Path;

use chrono::{NaiveDateTime, Utc};
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::{
    constants,
    models::task::Task,
    services::account_service,
};

pub fn add_task(content: &str) -> Result<()>{
    let path = Path::new(constants::TASKS_FILE);
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&path);

    match get_metadata(path) {
        Ok(mut tasks) => {
            let id = tasks.len()  as i32 + constants::SINGULAR_PLURAL_THRESHOLD;
            let user_id = account_service::get_current_user().unwrap().id;
            let task = Task::new(content, id, user_id);
            tasks.push(task);

            write_to_file(&path, &tasks);

            println!("{id}. {content}", id = id, content = content);
            println!();
            println!("Item {id} added", id = id);
        },
        Err(_) => {
            println!("{}", constants::GET_FILE_DATA_WRONG);
        }
    }

    Ok(())
}

pub fn finish_task(id: i32) -> Result<()>{
    let path = Path::new(constants::TASKS_FILE);

    match get_metadata(path) {
        Ok(mut tasks) => {
            let user_id = account_service::get_current_user().unwrap().id;
            match tasks.iter_mut().find(|task| task.id == id && task.user_id == user_id) {
                Some(task) => {
                    task.finished = true;
                    task.updated_at = Utc::now().naive_utc();
                }
                None => {
                    println!("{}", constants::TASK_DOES_NOT_EXIST);
                    return Ok(());
                }
            }

            write_to_file(&path, &tasks);

            println!("Item {id} done.", id=id);
        },
        Err(_) => {
            println!("{}", constants::GET_FILE_DATA_WRONG);
        }
    }

    Ok(())
}

pub fn get_tasks() -> Result<()> {
    let path = Path::new(constants::TASKS_FILE);

    match get_metadata(path) {
        Ok(tasks) => {
            let user_id = account_service::get_current_user().unwrap().id;
            let sorted_tasks = tasks
                .clone()
                .into_iter()
                .filter(|task| task.user_id == user_id)
                .sorted_by_key(|task| task.finished);
            let total = sorted_tasks.len();
            let mut finished_count = 0;

            for task in sorted_tasks {
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
        Err(_) => {
            println!("{}", constants::GET_FILE_DATA_WRONG);
        }
    }

    Ok(())
}

pub fn get_unfinished_tasks() -> Result<()> {
    let path = Path::new(constants::TASKS_FILE);

    match get_metadata(path) {
        Ok(tasks) => {
            let user_id = account_service::get_current_user().unwrap().id;
            let unfinished_tasks: Vec<Task> = tasks
                .iter()
                .filter(|task| !task.finished && task.user_id == user_id)
                .cloned()
                .collect();
            let total = unfinished_tasks.len();

            for task in unfinished_tasks.iter() {
                println!("{id}. {content}", id = task.id, content = task.content);
            }

            println!();
            let word = get_singular_plural(total, "item".to_string());
            println!("Total: {total} {word}", total = total, word = word);
        },
        Err(_) => {
            println!("{}", constants::GET_FILE_DATA_WRONG);
        }
    }

    Ok(())
}

pub fn export_tasks(file_name: &str) -> Result<()> {
    if fs::metadata(constants::TASKS_FILE).is_err() {
        println!("{}", constants::NO_TASK);
    }

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

    // 只导出属于自己的数据
    let user_id = account_service::get_current_user().unwrap().id;
    let original_path = Path::new(constants::TASKS_FILE);
    match get_metadata(original_path) {
        Ok(tasks) => {
            let download_tasks: Vec<Task> = tasks
                .iter()
                .filter(|task| task.user_id == user_id)
                .cloned()
                .collect();
            write_to_file(download_path, &download_tasks);

            let total = download_tasks.len();
            let word = get_singular_plural(total, "item".to_string());
            println!("Export success. {total} {word} exported.", total = total, word = word);
        }
        Err(_) => {
            println!("{}",constants::GET_FILE_DATA_WRONG);
        }
    }
    Ok(())
}

pub fn import_tasks(file_name: &str) -> Result<()> {
    if !file_name.ends_with(constants::IMPORT_FILE_SUFFIX) {
        println!("{}",constants::ASK_FOR_JSON_FILE);
        return Ok(());
    }
    if fs::metadata(file_name).is_err() {
        println!("{}",constants::FILE_NOT_EXIST);
        return Ok(());
    }

    let import_file_path = Path::new(file_name);
    match get_metadata(import_file_path) {
        Ok(waiting_tasks) => {
            // 去重
            let purified_tasks = waiting_tasks
                .iter()
                .unique_by(|task| &task.user_id)
                .unique_by(|task| &task.content)
                .unique_by(|task| &task.finished)
                .collect::<Vec<_>>();
            let success_count = purified_tasks.len();

            let task_file_path = Path::new(constants::TASKS_FILE);
            if fs::metadata(constants::TASKS_FILE).is_err() {
                OpenOptions::new()
                    .read(true)
                    .write(true)
                    .create(true)
                    .open(&task_file_path);
            }

            let mut tasks = get_metadata(task_file_path).unwrap();
            for task in purified_tasks.iter() {
                tasks.push(task.clone().clone());
            }

            write_to_file(&task_file_path, &tasks);

            println!("Import success, success {}", success_count);
        },
        Err(_) => {
            println!("{}",constants::GET_FILE_DATA_WRONG);
        }
    }


    Ok(())
}

fn get_metadata(path: &Path) -> Result<Vec<Task>> {
    let string_data = fs::read_to_string(&path).expect(constants::UNABLE_TO_READ_FILE);
    let mut tasks: Vec<Task> = vec![];
    if fs::metadata(&path).unwrap().len() != 0 {
        tasks = serde_json::from_str(&string_data)?;
    }

    Ok(tasks)
}

fn write_to_file(path: &Path, tasks: &Vec<Task>) -> Result<()>{
    let json: String = serde_json::to_string(tasks)?;
    fs::write(path, &json).expect(constants::UNABLE_WRITE_TO_FILE);

    Ok(())
}

fn get_singular_plural(count: usize, word: String) -> String {
    if count as i32 > constants::SINGULAR_PLURAL_THRESHOLD {
        format!("{}s", word)
    } else {
        format!("{}", word)
    }
}