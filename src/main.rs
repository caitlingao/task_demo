// #[macro_use]
// extern crate r2d2_redis;
//
// use std::thread;
//
// use r2d2_redis::{r2d2, redis, RedisConnectionManager};
// use r2d2_redis::redis::Commands;
//
//
// fn main() {
//     let manager = RedisConnectionManager::new("redis://redis:123456@localhost:6379").expect("set");
//     let pool = r2d2::Pool::builder()
//         .build(manager)
//         .unwrap();
//     let mut conn = pool.get().unwrap();
//     let n: i64 = conn.incr("counter", 1).unwrap();
//     println!("Counter increased to {}", n);
//
//     // let mut handles = vec![];
//     //
//     // for _i in 0..10i32 {
//     //     let pool = pool.clone();
//     //     handles.push(thread::spawn(move || {
//     //         let mut conn = pool.get().unwrap();
//     //         let n: i64 = conn.incr("counter", 1).unwrap();
//     //         println!("Counter increased to {}", n);
//     //     }));
//     // }
//     //
//     // for h in handles {
//     //     h.join().unwrap();
//     // }
// }
#![feature(plugin, decl_macro, proc_macro_hygiene)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate redis;
extern crate r2d2_redis;

extern crate dotenv;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use std::{fs, env};
use std::io::{stdin, stdout, Write};

use clap::ArgMatches;
use dotenv::dotenv;
use diesel::prelude::*;
use diesel::pg::PgConnection;

use constants::TMP_DIR;
use services::*;
use controllers::tasks_controller;

mod cli;
mod config;
mod constants;
mod controllers;
mod models;
mod schema;
mod services;
mod utils;

fn rocket() -> rocket::Rocket {
    dotenv().ok();

    // let database_url = env::var("DATABASE_URL").expect("set DATABaSe");
    // let pool = config::db::migrate_and_config_db(&database_url);
    let pool = config::db::migrate_and_config_db();
    let redis_pool = config::db::redis_pool();

    rocket::ignite()
        .manage(pool)
        .mount(
            "/api/v1/",
            routes![tasks_controller::index],
        )
}
fn main() {
    rocket().launch();

    // let database_url = env::var("DATABASE_URL").expect("set DATABaSe");
//     let conn = PgConnection::establish(&database_url).unwrap();
//
//     // 预先创建 tmp 目录，以便于存储 task 数据及登录缓存数据
//     if fs::metadata(TMP_DIR).is_err() {
//         fs::create_dir(TMP_DIR);
//     }
//
//     let matches = cli::build_cli().get_matches();
//     match matches.subcommand() {
//         Some(sub_cmd) => {
//             if sub_cmd.0 != "login" {
//                 if account_service::get_current_user().is_none() {
//                     println!("{}", constants::ASK_FOR_LOGIN);
//                     return;
//                 }
//             }
//             run_command(sub_cmd, &conn)
//         },
//         _ => unreachable!("The cli parser should prevent reaching here"),
//     }
// }
//
// fn run_command(cmd: (&str, &ArgMatches), conn: &PgConnection) {
//     match (cmd.0, cmd.1) {
//        ("login", arg) => {
//            println!("Password:");
//
//            stdout().flush().unwrap();
//            let mut input = String::new();
//            stdin().read_line(&mut input).unwrap();
//            let password = input.trim();
//            let email = arg.value_of("user").unwrap();
//
//            account_service::login(email, password, conn);
//        },
//        ("logout", _) => {
//            account_service::logout();
//        },
//        ("add", arg) => {
//            let content = arg.value_of("content").unwrap();
//            task_service::add_task(content, conn);
//        }
//        ("done", arg) => {
//            let arg_id = arg.value_of("id").unwrap();
//            let task_id = arg_id.parse::<i32>().unwrap();
//            task_service::finish_task(task_id, conn);
//        }
//        ("list", arg) => {
//            match arg.occurrences_of("all") {
//                1 => task_service::get_tasks(conn),
//                0 | _ => task_service::get_unfinished_tasks(conn),
//            };
//        }
//         ("export", arg) => {
//             let target = arg.value_of("target").unwrap();
//             task_service::export_tasks(target, conn);
//         }
//         ("import", arg) => {
//             let file = arg.value_of("file").unwrap();
//             task_service::import_tasks(file, conn);
//         }
//         ("init", _) => {
//             account_service::import_users(conn);
//             task_service::init_tasks(conn);
//         },
//        _ => unreachable!("The cli parser should prevent reaching here"),
//    }
}
