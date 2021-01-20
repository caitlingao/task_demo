use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use diesel::{prelude::*, pg::PgConnection};

use crate::{
    schema::tasks::{self, dsl::*},
    config::db::Connection,
    models::user::User,
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

#[derive(Insertable, Serialize, Deserialize, Clone, Debug)]
#[table_name = "tasks"]
pub struct TaskDTO {
    pub user_id: i32,
    pub content: String,
    pub finished: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NewTask {
    pub content: String,
}

impl Task {
    pub fn index(e: &str, conn: &Connection) -> QueryResult<Vec<Task>> {
        let user = User::find_user_by_email(e, conn).unwrap();
        tasks
            .filter(user_id.eq(user.id))
            .order(id.asc())
            .order(finished.asc())
            .load::<Task>(conn)
    }

    pub fn insert(e: &str, task: NewTask, conn: &Connection) -> QueryResult<usize> {
        let user = User::find_user_by_email(e, conn).unwrap();
        let task = TaskDTO {
            user_id: user.id,
            content: task.content,
            finished: false
        };
        diesel::insert_into(tasks)
            .values(&task)
            .execute(conn)
    }

    pub fn finish(e: &str, task_id: i32, conn: &Connection) -> QueryResult<usize> {
        let user = User::find_user_by_email(e, conn).unwrap();
        diesel::update(
           tasks
               .filter(id.eq(task_id))
               .filter(user_id.eq(user.id))
        ).set(finished.eq(true))
            .execute(conn)
    }

    // pub fn find_all(current_user_id: i32, conn: &PgConnection) -> QueryResult<Vec<Task>> {
    //     tasks
    //         .filter(user_id.eq(current_user_id))
    //         .order(id.asc())
    //         .order(finished.asc())
    //         .load::<Task>(conn)
    // }
    //
    // pub fn find_unfinished(current_user_id: i32, conn: &PgConnection) -> QueryResult<Vec<Task>> {
    //     tasks
    //         .filter(finished.eq(false))
    //         .filter(user_id.eq(current_user_id))
    //         .order(id.asc())
    //         .load::<Task>(conn)
    // }
    //
    // pub fn finish_task(task_id: i32, current_user_id: i32, conn: &PgConnection) -> QueryResult<usize> {
    //     diesel::update(tasks.find(task_id).find(current_user_id))
    //         .set(finished.eq(true))
    //         .execute(conn)
    // }
    //
    // pub fn mul_insert(mul_tasks: Vec<TaskDTO>, conn: &PgConnection) -> QueryResult<usize> {
    //     diesel::insert_into(tasks)
    //         .values(&mul_tasks)
    //         .execute(conn)
    // }
}
