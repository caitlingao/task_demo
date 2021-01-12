use clap::{App, Arg};

pub fn build_cli() -> App<'static> {
    App::new("task")
        .about("A fictional versioning CLI")
        .subcommand(
            App::new("login")
                .about("user login")
                .arg(
                    Arg::new("user")
                        .short('u')
                        .long("user")
                        .about("user's email")
                        .takes_value(true)
                )
        )
        .subcommand(
            App::new("logout")
                .about("user logout")
        )
        .subcommand(
            App::new("add")
            .about("add task")
            .arg(
                Arg::new("content")
                .about("The content to add")
                .required(true)
            )
        )
        .subcommand(
            App::new("done")
            .about("finish task")
            .arg(
                Arg::new("id")
                .about("finished task id")
                .required(true)
            )
        )
        .subcommand(
            App::new("list")
            .about("show task list")
            .arg(
                Arg::new("all")
                .long("all")
                .about("show all task list")
            ),
        )
}