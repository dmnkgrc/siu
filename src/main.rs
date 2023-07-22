use std::error::Error;

use clap::Parser;
use owo_colors::OwoColorize;

pub mod db;
pub mod diff;
pub mod models;
pub mod projects;
pub mod schema;
pub mod shell;
pub mod tools;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    project: String,
    #[arg(short, long, help = "Resets a project's progress")]
    reset: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    projects::init();
    let cli = Cli::parse();
    let project = projects::get(&cli.project).unwrap();
    if cli.reset {
        if let Err(e) = project.reset() {
            println!("\n{}", e.red());
        }
        return Ok(());
    }
    let title = format!("Will setup \"{}\"", project.options.name);
    println!("\n\n{}", title.green().bold());
    println!("{}\n\n", project.options.description.cyan());
    if let Err(e) = project.setup() {
        println!("\n{}", e.red());
    }
    Ok(())
}
