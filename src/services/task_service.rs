use std::fs::{self, OpenOptions};
use std::io::{Result};
use std::path::Path;

use chrono::{NaiveDateTime, Utc};
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::{
    constants,
    models::task::Task,
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
            let task = Task::new(content, id);
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
            match tasks.iter_mut().find(|task| task.id == id) {
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
            let sorted_tasks = tasks
                .clone()
                .into_iter()
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
            let unfinished_tasks: Vec<Task> = tasks
                .iter()
                .filter(|task| !task.finished)
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