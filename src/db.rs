use std::fs;
use std::{env, path::Path};

use diesel::prelude::*;

use crate::models::{NewProject, NewProjectProgress, Project, ProjectProgress};

pub struct Db {
    conn: SqliteConnection,
}

impl Db {
    pub fn new() -> Self {
        let home = env::var("HOME").unwrap();
        let cache_path = format!("{}/.cache", home);
        let cache_path_exists = Path::new(&cache_path).is_dir();
        if !cache_path_exists {
            match fs::create_dir_all(&cache_path) {
                Ok(_) => {}
                Err(e) => {
                    panic!("Failed to create .cache dir: {}", e);
                }
            }
        }
        let database_url = format!("{}/siu.sqlite", &cache_path);

        let conn = SqliteConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

        Self { conn }
    }

    pub fn run_migrations(&mut self) {
        use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

        const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

        self.conn
            .run_pending_migrations(MIGRATIONS)
            .expect("Failed to run migrations");
    }

    pub fn get_project(&mut self, project_name: &str) -> Option<Project> {
        use crate::schema::projects::dsl::*;
        projects
            .filter(name.eq(project_name))
            .first(&mut self.conn)
            .optional()
            .unwrap()
    }

    pub fn create_project(&mut self, project_name: &str, project_path: &str) -> Project {
        use crate::schema::projects;
        let new_project = NewProject {
            name: project_name,
            path: project_path,
        };

        diesel::insert_into(projects::table)
            .values(&new_project)
            .returning(Project::as_returning())
            .get_result(&mut self.conn)
            .expect("Error saving new project")
    }

    pub fn get_project_progress(&mut self, proj: &Project) -> ProjectProgress {
        use crate::schema::projects_progress::dsl::*;
        match ProjectProgress::belonging_to(proj)
            .first(&mut self.conn)
            .optional()
            .unwrap()
        {
            Some(p) => p,
            None => {
                let new_project_progress = NewProjectProgress {
                    project_id: proj.id,
                    step: 0,
                    tool: 0,
                    tool_step: 0,
                };
                diesel::insert_into(projects_progress)
                    .values(&new_project_progress)
                    .get_result(&mut self.conn)
                    .expect("Error saving new project progress")
            }
        }
    }

    pub fn update_project_progress(
        &mut self,
        proj: &Project,
        step: &i32,
        tool: &i32,
        tool_step: &i32,
    ) -> ProjectProgress {
        use crate::schema::projects_progress;
        let progress = ProjectProgress::belonging_to(proj)
            .select(ProjectProgress::as_select())
            .first(&mut self.conn)
            .unwrap();
        diesel::update(&progress)
            .set((
                projects_progress::step.eq(step),
                projects_progress::tool.eq(tool),
                projects_progress::tool_step.eq(tool_step),
            ))
            .get_result(&mut self.conn)
            .unwrap()
    }
}

impl Default for Db {
    fn default() -> Self {
        Self::new()
    }
}
