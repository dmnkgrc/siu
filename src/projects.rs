use dialoguer::theme::ColorfulTheme;
use dialoguer::Confirm;
use owo_colors::OwoColorize;
use url::Url;

use std::process::exit;
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
                let theme = ColorfulTheme::default();
                println!("\nAre you ready to continue?");
                match Confirm::with_theme(&theme)
                    .with_prompt(
                        "Press Enter or 'y' to continue or Esc, 'q' or 'n' to exit and finish later",
                    )
                    .default(true)
                    .interact_opt()
                    .unwrap()
                {
                    Some(true) => Ok(false),
                    Some(false) => exit(0),
                    None => exit(0),
                }
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
pub struct YamlConfiguration {
    pub name: String,
    pub description: String,
    pub steps: Vec<StepConfiguration>,
}

pub struct ProjectConfiguration {
    pub options: YamlConfiguration,
    // Actual path in the system
    pub path: Option<String>,
    pub url: Option<String>,
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
        let step = &self.options.steps[index];
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
        if index < self.options.steps.len() - 1 {
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
        match db.get_project(&self.options.name) {
            Some(project) => (project.clone(), db.get_project_progress(&project)),
            None => {
                let path = if let Some(p) = &self.path {
                    p
                } else if let Some(u) = &self.url {
                    u
                } else {
                    panic!("Projects must have either a path or a url");
                };
                let project = db.create_project(&self.options.name, path);
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
                if progress.step as usize > self.options.steps.len() - 1 {
                    db.update_project_progress(&project, &0, &0, &0);
                }
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    pub fn reset(&self) -> Result<(), String> {
        let mut db = Db::default();
        let project = db.get_project(&self.options.name).unwrap();
        db.update_project_progress(&project, &0, &0, &0);
        Ok(())
    }
}

fn get_projects_path() -> String {
    match home::home_dir() {
        Some(path) => env::var("SIU_PROJECTS_PATH")
            .unwrap_or_else(|_| path.to_string_lossy().to_string() + "/.siu/projects"),
        None => panic!("Home directory not found, please set the SIU_PROJECTS_PATH env variable"),
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

fn parse_project_file(
    project_file: &fs::File,
    file_path: &Path,
) -> Result<ProjectConfiguration, String> {
    match serde_yaml::from_reader(project_file) {
        Ok(options) => Ok(ProjectConfiguration {
            options,
            path: Some(file_path.to_str().unwrap().to_string()),
            url: None,
        }),
        Err(e) => Err(format!("Failed to parse project file: {}", e)),
    }
}

fn parse_project_text(project: &str, url: &str) -> Result<ProjectConfiguration, String> {
    match serde_yaml::from_str(project) {
        Ok(options) => Ok(ProjectConfiguration {
            options,
            path: None,
            url: Some(url.to_string()),
        }),
        Err(e) => Err(format!("Failed to parse project file: {}", e)),
    }
}

fn parse_project_file_from_path(path: &Path) -> Result<ProjectConfiguration, String> {
    match fs::File::open(path) {
        Ok(file) => parse_project_file(&file, path),
        Err(e) => Err(format!("Failed to open project file: {}", e)),
    }
}

fn has_yaml_extension(s: &str) -> bool {
    s.ends_with(".yaml") || s.ends_with(".yml")
}

fn is_yaml(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(has_yaml_extension)
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

#[tokio::main]
pub async fn get(name: &str) -> Result<ProjectConfiguration, String> {
    if Url::parse(name).is_ok() {
        println!(
            "{}",
            "Project URL detected, fetching project from url...".bold()
        );
        let res = reqwest::get(name)
            .await
            .expect("Failed to fetch project from url");
        let body = res.text().await.expect("Failed to read project response");
        return parse_project_text(&body, name);
    }
    let name_path = Path::new(name);
    if name_path.exists() && has_yaml_extension(name) {
        return parse_project_file_from_path(name_path);
    }
    let projects_path = get_projects_path();
    let mut path_buf = Path::new(&projects_path).join(name);
    path_buf.set_extension("yaml");
    parse_project_file_from_path(path_buf.as_path())
}
