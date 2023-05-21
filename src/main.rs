use std::{collections::BTreeMap, env, ffi::OsString, fs, path::Path};

use clap::{arg, Command, Parser};

#[derive(Parser, Debug)]
struct Cli {
    // The action to use, can be `setup` or `update`
    action: String,
    // The project to use for the action
    project: String,
}

fn get_projects_path() -> String {
    match home::home_dir() {
        Some(path) => {
            return env::var("SMU_PROJECTS_PATH").unwrap_or_else(|_| {
                String::from(path.to_string_lossy().to_string() + "/.smu/projects")
            });
        }
        None => panic!("Home directory not found, please set the SMU_PROJECTS_PATH env variable"),
    }
}

fn init_projects(projects_path: &String) {
    let projects_path_exists = Path::new(projects_path).is_dir();
    if !projects_path_exists {
        fs::create_dir_all(projects_path).expect("Failed to create projects dir");
    }
}

fn get_project(
    projects_path: &String,
    project: &String,
) -> Result<BTreeMap<String, String>, Box<dyn std::error::Error>> {
    let mut path = Path::new(projects_path).join(project);
    path.set_extension("yaml");
    let file = std::fs::File::open(path)?;
    let result: BTreeMap<String, String> = serde_yaml::from_reader(file)?;
    println!("Result: {:?}", result);
    Ok(result)
}

fn cli() -> Command {
    Command::new("smu")
        .about("A CLI to setup everything you need to run your projects")
        .subcommand_required(true)
        .subcommand(
            Command::new("setup")
                .about("Setup the specified project")
                .arg(arg!(<PROJECT> "The project to setup")),
        )
}

fn main() {
    let projects_path = get_projects_path();
    init_projects(&projects_path);
    let matches = cli().get_matches();
    match matches.subcommand() {
        Some(("setup", sub_matches)) => {
            let project_name = sub_matches.get_one::<String>("PROJECT").expect("Required");
            println!(
                "Setting up project: {}",
                sub_matches.get_one::<String>("PROJECT").expect("Required")
            );
            let project = get_project(&projects_path, project_name).expect("Not found");
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
