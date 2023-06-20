use crate::schema::{projects, projects_progress};
use diesel::prelude::*;

#[derive(Queryable, Selectable, Identifiable, Clone)]
#[diesel(table_name = projects)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub path: String,
}

#[derive(Insertable)]
#[diesel(table_name = projects)]
pub struct NewProject<'a> {
    pub name: &'a str,
    pub path: &'a str,
}

#[derive(Queryable, Selectable, Associations, Identifiable, Clone, Copy)]
#[diesel(table_name = projects_progress)]
#[diesel(belongs_to(Project))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ProjectProgress {
    pub id: i32,
    pub project_id: i32,
    pub step: i32,
    pub tool: i32,
    pub tool_step: i32,
}

#[derive(Insertable)]
#[diesel(table_name = projects_progress)]
pub struct NewProjectProgress {
    pub project_id: i32,
    pub step: i32,
    pub tool: i32,
    pub tool_step: i32,
}
