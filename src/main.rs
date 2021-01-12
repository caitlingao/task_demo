use std::fs;
use std::io::{stdin, stdout, Write};

use clap::ArgMatches;

use constants::TMP_DIR;
use services::{task_service, account_service};

mod cli;
mod constants;
mod models;
mod services;

fn main() {
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
            run_command(sub_cmd)
        },
        _ => unreachable!("The cli parser should prevent reaching here"),
    }
}

fn run_command(cmd: (&str, &ArgMatches)) {
    match (cmd.0, cmd.1) {
       ("login", arg) => {
           println!("Password:");

           stdout().flush().unwrap();
           let mut input = String::new();
           stdin().read_line(&mut input).unwrap();
           let password = input.trim();
           let email = arg.value_of("user").unwrap();

           account_service::login(email, password);
       },
       ("logout", _) => {
           account_service::logout();
       },
       ("add", arg) => {
           let content = arg.value_of("content").unwrap();
           task_service::add_task(content);
       }
       ("done", arg) => {
           let arg_id = arg.value_of("id").unwrap();
           if let Ok(id) = arg_id.parse::<i32>() {
               task_service::finish_task(id);
           } else {
               println!("Invalid id");
           }
       }
       ("list", arg) => {
           match arg.occurrences_of("all") {
               0 => task_service::get_unfinished_tasks(),
               1 => task_service::get_tasks(),
               _ => task_service::get_unfinished_tasks(),
           };
       }
        ("export", arg) => {
            let target = arg.value_of("target").unwrap();
            task_service::export_tasks(target);
        }
        ("import", arg) => {
            let file = arg.value_of("file").unwrap();
            task_service::import_tasks(file);
        }
       _ => unreachable!("The cli parser should prevent reaching here"),
   }
}
