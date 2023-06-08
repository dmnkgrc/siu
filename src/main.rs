use std::error::Error;

pub mod cli;
pub mod projects;
pub mod shell;
pub mod tools;

fn main() -> Result<(), Box<dyn Error>> {
    projects::init();
    let mut terminal = cli::setup_terminal()?;
    cli::run(&mut terminal)?;
    cli::restore_terminal()?;
    Ok(())
}
