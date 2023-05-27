use std::{env, fs, path::Path};

use serde::{Deserialize, Serialize};

use crate::tools::homebrew;
use crate::tools::node;
use crate::tools::tool::Tool;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
enum SupportedTool {
    Homebrew,
    Node,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Step {
    description: String,
    tool: Option<SupportedTool>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Project {
    name: String,
    description: String,
    steps: Vec<Step>,
}

fn get_projects_path() -> String {
    match home::home_dir() {
        Some(path) => env::var("SMU_PROJECTS_PATH")
            .unwrap_or_else(|_| path.to_string_lossy().to_string() + "/.smu/projects"),
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
    println!("\n\nSetting up project: {}\n", project.name);
    println!("{}\n", project.description);
    (project.steps.iter().enumerate()).for_each(|(i, step)| {
        println!("\n{}: {}\n", i + 1, step.description);
        match step.tool {
            Some(SupportedTool::Homebrew) => {
                let brew = homebrew::Homebrew {};
                brew.install().expect("Failed to install Homebrew");
            }
            Some(SupportedTool::Node) => {
                let node = node::Node {};
                node.install().expect("Failed to install Node");
            }
            None => {
                unimplemented!();
            }
        };
    });
    Ok(())
}
