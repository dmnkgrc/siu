use std::process::{Child, Command, Stdio};
use std::{env, fs, path::Path};

use ratatui::widgets::ListItem;
use serde::{Deserialize, Serialize};
use walkdir::DirEntry;
use walkdir::WalkDir;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct RunTool {
    brew: Option<String>,
}

impl RunTool {
    pub fn get_install_cmd(self) -> Child {
        if let Some(brew) = self.brew {
            let brew_arg = brew.as_str();
            return Command::new("brew")
                .args(["install", brew_arg])
                .stdout(Stdio::null())
                .spawn()
                .unwrap();
        };
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

impl<'a> From<Project> for ListItem<'a> {
    fn from(value: Project) -> Self {
        ListItem::new(value.name)
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

// fn install_tool(tool: &dyn Tool) -> Result<(), String> {
//     match tool.install() {
//         Ok(_) => Ok(()),
//         Err(e) => Err(format!("Failed to install tool {}: {}", tool.name(), e)),
//     }
// }

// fn setup_step(step: &Step) -> Result<(), String> {
//     match step.tool {
//         Some(SupportedTool::Homebrew) => install_tool(&homebrew::Homebrew {}),
//         Some(SupportedTool::Node) => install_tool(&node::Node {}),
//         None => {
//             unimplemented!();
//         }
//     }
// }
//
// pub fn setup(project: &Project) -> Result<(), String> {
//     println!("\n\nSetting up project: {}\n", project.name);
//     println!("{}\n", project.description);
//     (project.steps.iter().enumerate()).for_each(|(i, step)| {
//         println!("\n{}: {}\n", i + 1, step.description);
//         match setup_step(step) {
//             Ok(_) => {}
//             Err(e) => {
//                 panic!("Failed to setup step {}: {}", i + 1, e)
//             }
//         }
//     });
//     Ok(())
// }
