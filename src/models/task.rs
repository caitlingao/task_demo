use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use diesel::prelude::*;

use crate::{
    config::db::Connection,
    schema::tasks::{self, dsl::*},
};

#[derive(Queryable, Clone, Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: i32,
    pub user_id: i32,
    pub content: String,
    pub finished: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize, Clone,Debug)]
#[table_name = "tasks"]
pub struct TaskDTO {
    pub user_id: i32,
    pub content: String,
    pub finished: bool,
}

pub struct NewTask {
    pub content: String,
}

impl Task {
    pub fn find_all(current_user_id: i32, conn: &Connection) -> QueryResult<Vec<Task>> {
        tasks
            .filter(user_id.eq(current_user_id))
            .order(id.asc())
            .order(finished.asc())
            .load::<Task>(conn)
    }

    pub fn find_unfinished(current_user_id: i32, conn: &Connection) -> QueryResult<Vec<Task>> {
        tasks
            .filter(finished.eq(false))
            .filter(user_id.eq(current_user_id))
            .order(id.asc())
            .load::<Task>(conn)
    }

    pub fn insert(task: TaskDTO, conn: &Connection) -> QueryResult<usize> {
        diesel::insert_into(tasks)
            .values(&task)
            .execute(conn)
    }

    pub fn finish_task(task_id: i32, current_user_id: i32, conn: &Connection) -> QueryResult<usize> {
        diesel::update(tasks.find(task_id).find(current_user_id))
            .set(finished.eq(true))
            .execute(conn)
    }

    pub fn mul_insert(mul_tasks: Vec<TaskDTO>, conn: &Connection) -> QueryResult<usize> {
        diesel::insert_into(tasks)
            .values(&mul_tasks)
            .execute(conn)
    }
}
