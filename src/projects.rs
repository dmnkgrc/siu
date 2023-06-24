use crossterm::event::poll;
use crossterm::event::read;
use crossterm::event::Event;
use crossterm::event::KeyCode;
use crossterm::terminal::disable_raw_mode;
use crossterm::terminal::enable_raw_mode;
use owo_colors::OwoColorize;

use std::time::Duration;
use std::{env, fs, path::Path};

use serde::{Deserialize, Serialize};
use walkdir::DirEntry;
use walkdir::WalkDir;

use crate::db::Db;
use crate::models::Project;
use crate::models::ProjectProgress;
use crate::tools::chezmoi::Chezmoi;
use crate::tools::homebrew::Homebrew;
use crate::tools::java11::Java11;
use crate::tools::pnpm::Pnpm;
use crate::tools::rbenv::Rbenv;
use crate::tools::types::Tool;
use crate::tools::yarn::Yarn;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(untagged)]
pub enum RunTool {
    Chezmoi { chezmoi: Chezmoi },
    Homebrew { brew: Homebrew },
    Java11 { java11: bool },
    Note { note: String },
    Pause { pause: bool },
    Pnpm { pnpm: Pnpm },
    Rbenv { rbenv: Rbenv },
    Yarn { yarn: Yarn },
}

impl RunTool {
    pub fn install(self, tool_step: usize) -> Result<bool, String> {
        match self {
            RunTool::Chezmoi { chezmoi } => chezmoi.install(tool_step),
            RunTool::Homebrew { brew } => brew.install(tool_step),
            RunTool::Java11 { java11: _ } => Java11 {}.install(tool_step),
            RunTool::Note { note } => {
                println!("\n\n{}\n", note.bold());
                Ok(false)
            }
            RunTool::Pause { pause: _ } => {
                enable_raw_mode().unwrap();
                println!("\n\n{}\n", "Press Enter to continue".bold());
                loop {
                    if (poll(Duration::from_millis(300))).unwrap() {
                        let event = read().unwrap();
                        if event == Event::Key(KeyCode::Enter.into()) {
                            break;
                        }
                    }
                }
                disable_raw_mode().unwrap();
                Ok(false)
            }
            RunTool::Pnpm { pnpm } => pnpm.install(tool_step),
            RunTool::Rbenv { rbenv } => rbenv.install(tool_step),
            RunTool::Yarn { yarn } => yarn.install(tool_step),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct StepConfiguration {
    pub description: String,
    pub run: Vec<RunTool>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct ProjectConfiguration {
    pub name: String,
    pub description: String,
    pub steps: Vec<StepConfiguration>,
}

impl ProjectConfiguration {
    fn run_step(
        &self,
        project: &Project,
        db: &mut Db,
        index: usize,
        tool: usize,
        tool_step: usize,
    ) -> Result<ProjectProgress, String> {
        let step = &self.steps[index];
        println!("\n{}", step.description.underline().bold());

        let tools: Vec<RunTool> = step.run.clone().drain(tool..).collect();

        for run in tools {
            match run.install(tool_step) {
                Ok(pause) => {
                    if pause {
                        let saved_progress = db.update_project_progress(
                            project,
                            &(index as i32),
                            &(tool as i32),
                            &(tool_step as i32 + 1),
                        );
                        return Ok(saved_progress);
                    }
                }
                Err(e) => return Err(e),
            }
            db.update_project_progress(project, &(index as i32), &(tool as i32 + 1), &0);
        }
        let new_progress = db.update_project_progress(project, &((index + 1) as i32), &0, &0);
        if index < self.steps.len() - 1 {
            return self.run_step(
                project,
                db,
                new_progress.step as usize,
                new_progress.tool as usize,
                new_progress.tool_step as usize,
            );
        }
        Ok(new_progress)
    }

    fn get_project_progress(&self, db: &mut Db) -> (Project, ProjectProgress) {
        match db.get_project(&self.name) {
            Some(project) => (project.clone(), db.get_project_progress(&project)),
            None => {
                let projects_path = get_projects_path();
                let mut path_buf = Path::new(&projects_path).join(&self.name);
                path_buf.set_extension("yaml");
                let project = db.create_project(&self.name, path_buf.to_str().unwrap());
                (project.clone(), db.get_project_progress(&project))
            }
        }
    }

    pub fn setup(&self) -> Result<(), String> {
        let mut db = Db::default();
        let (project, progress) = self.get_project_progress(&mut db);
        if progress.tool > 0 || progress.tool_step > 0 {
            println!("{}", "Picking up where you left off".green().bold());
        }
        let progress_result = self.run_step(
            &project,
            &mut db,
            progress.step as usize,
            progress.tool as usize,
            progress.tool_step as usize,
        );
        match progress_result {
            Ok(progress) => {
                if progress.step as usize > self.steps.len() - 1 {
                    db.update_project_progress(&project, &0, &0, &0);
                }
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    pub fn reset(&self) -> Result<(), String> {
        let mut db = Db::default();
        let project = db.get_project(&self.name).unwrap();
        db.update_project_progress(&project, &0, &0, &0);
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

fn parse_project_file(project_file: &fs::File) -> Result<ProjectConfiguration, String> {
    match serde_yaml::from_reader(project_file) {
        Ok(project) => Ok(project),
        Err(e) => Err(format!("Failed to parse project file: {}", e)),
    }
}

fn parse_project_file_from_path(path: &Path) -> Result<ProjectConfiguration, String> {
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

pub fn get_all() -> Result<Vec<ProjectConfiguration>, String> {
    let projects_path = get_projects_path();
    let mut projects: Vec<ProjectConfiguration> = Vec::new();

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

pub fn get(name: &String) -> Result<ProjectConfiguration, String> {
    let projects_path = get_projects_path();
    let mut path_buf = Path::new(&projects_path).join(name);
    path_buf.set_extension("yaml");
    parse_project_file_from_path(path_buf.as_path())
}
