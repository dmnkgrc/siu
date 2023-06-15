use std::error::Error;

use clap::Parser;
use owo_colors::OwoColorize;

pub mod projects;
pub mod shell;
pub mod tools;

#[derive(Parser)]
struct Cli {
    project: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    projects::init();
    let cli = Cli::parse();
    let project = projects::get(&cli.project).unwrap();
    let title = format!("Will setup \"{}\"", project.name);
    println!("\n\n{}", title.green().bold());
    println!("{}\n\n", project.description.cyan());
    if let Err(e) = project.setup() {
        println!("\n{}", e.red());
    }
    Ok(())
}
