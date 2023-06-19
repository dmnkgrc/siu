use std::env;

use rusqlite::Connection;

pub struct Db {
    conn: Connection,
}

impl Db {
    pub fn new() -> Self {
        let home = env::var("HOME").unwrap();
        let path = format!("{}/.cache/smu/smu.db3", home);

        let conn = Connection::open(path).expect("Failed to open database");

        conn.execute(
            "CREATE TABLE IF NOT EXISTS project (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                path TEXT NOT NULL UNIQUE,
            )",
            (),
        )
        .expect("Failed to setup database");

        conn.execute(
            "CREATE TABLE IF NOT EXISTS project_progress (
                project_id INTEGER UNIQUE,
                step INTEGER,
                sub_step INTEGER,
                FOREIGN KEY (project_id) REFERENCES project(id)
            )",
            (),
        )
        .expect("Failed to setup database");

        Self { conn }
    }
}

impl Default for Db {
    fn default() -> Self {
        Self::new()
    }
}
