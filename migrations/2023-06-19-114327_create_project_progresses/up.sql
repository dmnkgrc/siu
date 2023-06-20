CREATE TABLE IF NOT EXISTS projects_progress (
    id INTEGER NOT NULL PRIMARY KEY,
    project_id INTEGER NOT NULL UNIQUE,
    step INTEGER NOT NULL ,
    tool INTEGER NOT NULL ,
    tool_step INTEGER NOT NULL ,
    FOREIGN KEY (project_id) REFERENCES project(id)
)
