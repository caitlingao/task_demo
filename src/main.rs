use std::fs;
use std::error::Error;

use clap::ArgMatches;

use constants::TMP_DIR;
use services::task_service;

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
        Some(sub_cmd) => run_command(sub_cmd),
        _ => unreachable!("The cli parser should prevent reaching here"),
    }
}

fn run_command(cmd: (&str, &ArgMatches)) {
   match (cmd.0, cmd.1) {
       ("login", arg) => {
           println!("{:?}", arg)
       },
       ("logout", arg) => {
           println!("{:?}", arg)
       },
       ("add", arg) => {
           println!("{:?}", arg.value_of("content"));
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
       _ => unreachable!("The cli parser should prevent reaching here"),
   }
}
