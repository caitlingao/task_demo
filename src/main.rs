#[macro_use]
extern crate diesel;
extern crate dotenv;

use std::{fs, env};
use std::io::{stdin, stdout, Write};

use clap::ArgMatches;
use dotenv::dotenv;
use diesel::prelude::*;
use diesel::pg::PgConnection;

use constants::TMP_DIR;
use services::*;

mod cli;
mod config;
mod constants;
mod models;
mod schema;
mod services;

fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("set DATABaSe");
    let conn = PgConnection::establish(&database_url).unwrap();

    // 预先创建 tmp 目录，以便于存储 task 数据及登录缓存数据
    if fs::metadata(TMP_DIR).is_err() {
        fs::create_dir(TMP_DIR);
    }

    let matches = cli::build_cli().get_matches();
    match matches.subcommand() {
        Some(sub_cmd) => {
            if sub_cmd.0 != "login" {
                if account_service::get_current_user().is_none() {
                    println!("{}", constants::ASK_FOR_LOGIN);
                    return;
                }
            }
            run_command(sub_cmd, &conn)
        },
        _ => unreachable!("The cli parser should prevent reaching here"),
    }
}

fn run_command(cmd: (&str, &ArgMatches), conn: &PgConnection) {
    match (cmd.0, cmd.1) {
       ("login", arg) => {
           println!("Password:");

           stdout().flush().unwrap();
           let mut input = String::new();
           stdin().read_line(&mut input).unwrap();
           let password = input.trim();
           let email = arg.value_of("user").unwrap();

           account_service::login(email, password, conn);
       },
       ("logout", _) => {
           account_service::logout();
       },
       ("add", arg) => {
           let content = arg.value_of("content").unwrap();
           task_service::add_task(content, conn);
       }
       ("done", arg) => {
           let arg_id = arg.value_of("id").unwrap();
           let task_id = arg_id.parse::<i32>().unwrap();
           task_service::finish_task(task_id, conn);
       }
       ("list", arg) => {
           match arg.occurrences_of("all") {
               1 => task_service::get_tasks(conn),
               0 | _ => task_service::get_unfinished_tasks(conn),
           };
       }
        ("export", arg) => {
            let target = arg.value_of("target").unwrap();
            task_service::export_tasks(target, conn);
        }
        ("import", arg) => {
            let file = arg.value_of("file").unwrap();
            task_service::import_tasks(file, conn);
        }
        ("init", _) => {
            account_service::import_users(conn);
            task_service::init_tasks(conn);
        },
       _ => unreachable!("The cli parser should prevent reaching here"),
   }
}
