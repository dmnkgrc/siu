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
        match fs::create_dir_all(projects_path) {
            Ok(_) => {}
            Err(e) => {
                panic!("Failed to create projects dir: {}", e);
            }
        }
    }
}

fn parse_project_file(project_file: &fs::File) -> Result<Project, String> {
    match serde_yaml::from_reader(project_file) {
        Ok(project) => Ok(project),
        Err(e) => Err(format!("Failed to parse project file: {}", e)),
    }
}

pub fn get(name: &String) -> Result<Project, String> {
    let projects_path = get_projects_path();
    let mut path = Path::new(&projects_path).join(name);
    path.set_extension("yaml");
    match fs::File::open(path) {
        Ok(file) => parse_project_file(&file),
        Err(e) => Err(format!("Failed to open project file: {}", e)),
    }
}

fn install_tool(tool: &dyn Tool) -> Result<(), String> {
    match tool.install() {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to install tool {}: {}", tool.name(), e)),
    }
}

fn setup_step(step: &Step) -> Result<(), String> {
    match step.tool {
        Some(SupportedTool::Homebrew) => install_tool(&homebrew::Homebrew {}),
        Some(SupportedTool::Node) => install_tool(&node::Node {}),
        None => {
            unimplemented!();
        }
    }
}

pub fn setup(project: &Project) -> Result<(), String> {
    println!("\n\nSetting up project: {}\n", project.name);
    println!("{}\n", project.description);
    (project.steps.iter().enumerate()).for_each(|(i, step)| {
        println!("\n{}: {}\n", i + 1, step.description);
        match setup_step(step) {
            Ok(_) => {}
            Err(e) => {
                panic!("Failed to setup step {}: {}", i + 1, e)
            }
        }
    });
    Ok(())
}
