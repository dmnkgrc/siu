use owo_colors::OwoColorize;

use std::{env, fs, path::Path};

use serde::{Deserialize, Serialize};
use walkdir::DirEntry;
use walkdir::WalkDir;

use crate::tools::{Tool, Tools};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct RunTool {
    brew: Option<String>,
    pnpm: Option<String>,
    yarn: Option<String>,
}

impl RunTool {
    pub fn install(self) -> Result<(), String> {
        if let Some(brew) = self.brew {
            return brew.brew().install();
        } else if let Some(pnpm) = self.pnpm {
            return pnpm.pnpm().install();
        } else if let Some(yarn) = self.yarn {
            return yarn.yarn().install();
        }
        unreachable!();
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Step {
    pub description: String,
    pub run: Vec<RunTool>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Project {
    pub name: String,
    pub description: String,
    pub steps: Vec<Step>,
}

impl Project {
    pub fn setup(&self) -> Result<(), String> {
        for step in &self.steps {
            println!("{}\n", step.description.underline().bold());
            for run in &step.run {
                run.clone().install()?
            }
        }
        Ok(())
    }
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

fn parse_project_file_from_path(path: &Path) -> Result<Project, String> {
    match fs::File::open(path) {
        Ok(file) => parse_project_file(&file),
        Err(e) => Err(format!("Failed to open project file: {}", e)),
    }
}

fn is_yaml(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.ends_with(".yaml") || s.ends_with(".yml"))
        .unwrap_or(false)
}

pub fn get_all() -> Result<Vec<Project>, String> {
    let projects_path = get_projects_path();
    let mut projects: Vec<Project> = Vec::new();

    let mut walker = WalkDir::new(projects_path).into_iter();
    loop {
        let entry = match walker.next() {
            None => break,
            Some(Err(e)) => return Err(format!("Failed to walk projects directory: {}", e)),
            Some(Ok(entry)) => entry,
        };
        if !is_yaml(&entry) {
            continue;
        }
        match parse_project_file_from_path(entry.path()) {
            Ok(p) => projects.push(p),
            Err(e) => return Err(e),
        }
    }

    Ok(projects)
}

pub fn get(name: &String) -> Result<Project, String> {
    let projects_path = get_projects_path();
    let mut path_buf = Path::new(&projects_path).join(name);
    path_buf.set_extension("yaml");
    parse_project_file_from_path(path_buf.as_path())
}
