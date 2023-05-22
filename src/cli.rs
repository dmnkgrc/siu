use std::ffi::OsString;

use crate::projects;
use clap::{arg, Command, Parser};

#[derive(Parser, Debug)]
struct Cli {
    // The action to use, can be `setup` or `update`
    action: String,
    // The project to use for the action
    project: String,
}

fn get_command() -> Command {
    Command::new("smu")
        .about("A CLI to setup everything you need to run your projects")
        .subcommand_required(true)
        .subcommand(
            Command::new("setup")
                .about("Setup the specified project")
                .arg(arg!(<PROJECT> "The project to setup")),
        )
}

pub fn run() {
    let matches = get_command().get_matches();
    match matches.subcommand() {
        Some(("setup", sub_matches)) => {
            let project_name = sub_matches.get_one::<String>("PROJECT").expect("Required");
            println!(
                "Setting up project: {}",
                sub_matches.get_one::<String>("PROJECT").expect("Required")
            );
            let project = projects::get(project_name).expect("Not found");
            println!("Project found: {:?}", project);
        }
        Some((ext, sub_matches)) => {
            let args = sub_matches
                .get_many::<OsString>("")
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();
            println!("Calling out to {ext:?} with {args:?}");
        }
        None => unreachable!(),
    }
}
