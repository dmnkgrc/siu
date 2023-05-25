use std::{env, fs, path::Path};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
enum Tool {
    Homebrew,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Step {
    description: String,
    tool: Option<Tool>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Project {
    name: String,
    description: String,
    steps: Vec<Step>,
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

pub fn init() {
    let projects_path = get_projects_path();
    let projects_path_exists = Path::new(&projects_path).is_dir();
    if !projects_path_exists {
        fs::create_dir_all(projects_path).expect("Failed to create projects dir");
    }
}

pub fn get(name: &String) -> Result<Project, Box<dyn std::error::Error>> {
    let projects_path = get_projects_path();
    let mut path = Path::new(&projects_path).join(name);
    path.set_extension("yaml");
    let file = std::fs::File::open(path)?;
    let result: Project = serde_yaml::from_reader(file)?;
    Ok(result)
}

pub fn setup(project: &Project) -> Result<(), &'static str> {
    println!("\n\nSetting up project: {}\n\n", project.name);
    println!("{}\n\n", project.description);
    (project.steps.iter().enumerate())
        .into_iter()
        .for_each(|(i, step)| {
            println!("{}: {}\n", i + 1, step.description);
        });
    Ok(())
}
