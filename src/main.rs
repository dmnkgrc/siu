use std::{error::Error, time::Duration};

use cli::CliApp;

pub mod cli;
pub mod projects;
pub mod shell;
pub mod tools;

fn main() -> Result<(), Box<dyn Error>> {
    projects::init();
    // create cli app and run it
    let mut terminal = cli::setup_terminal()?;
    let tick_rate = Duration::from_millis(250);
    let cli_app = CliApp::default();
    cli::run_app(&mut terminal, cli_app, tick_rate)?;
    cli::restore_terminal()?;
    Ok(())
}
