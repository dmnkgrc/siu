use std::{
    error::Error,
    io::{self, Stdout},
    time::Duration,
};

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Padding},
    Terminal, TerminalOptions, Viewport,
};

pub fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, Box<dyn Error>> {
    let stdout = io::stdout();
    enable_raw_mode()?;
    let terminal = Terminal::with_options(
        CrosstermBackend::new(stdout),
        TerminalOptions {
            viewport: Viewport::Inline(8),
        },
    )?;
    Ok(terminal)
}

pub fn restore_terminal() -> Result<(), Box<dyn Error>> {
    disable_raw_mode()?;
    Ok(())
}

pub fn run(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<(), Box<dyn Error>> {
    loop {
        terminal.draw(|frame| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints([Constraint::Percentage(90), Constraint::Percentage(10)].as_ref())
                .split(frame.size());
            let title = "Which project do you want to setup?";
            let padding = Padding::uniform(2);
            let block = Block::default()
                .padding(padding)
                .title_alignment(Alignment::Center)
                .title(Span::styled(
                    title,
                    Style::default()
                        .add_modifier(Modifier::BOLD)
                        .fg(Color::Blue),
                ));
            frame.render_widget(block, chunks[0]);
        })?;
        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                if KeyCode::Char('q') == key.code {
                    break;
                }
            }
        }
    }
    Ok(())
}
// use std::ffi::OsString;
//
// use crate::projects;
// use clap::{arg, Command, Parser};
//
// #[derive(Parser, Debug)]
// struct Cli {
//     // The action to use, can be `setup` or `update`
//     action: String,
//     // The project to use for the action
//     project: String,
// }
//
// fn get_command() -> Command {
//     Command::new("smu")
//         .about("A CLI to setup everything you need to run your projects")
//         .subcommand_required(true)
//         .subcommand(
//             Command::new("setup")
//                 .about("Setup the specified project")
//                 .arg(arg!(<PROJECT> "The project to setup")),
//         )
// }
//
// pub fn run() {
//     let matches = get_command().get_matches();
//     match matches.subcommand() {
//         Some(("setup", sub_matches)) => {
//             let project_name = sub_matches.get_one::<String>("PROJECT").expect("Required");
//             let project = projects::get(project_name).expect("Not found");
//             let result = projects::setup(&project);
//             match result {
//                 Ok(_) => println!("Finished setting up project! ✅"),
//                 Err(e) => println!("Error setting up project: {e:?}"),
//             }
//         }
//         Some((ext, sub_matches)) => {
//             let args = sub_matches
//                 .get_many::<OsString>("")
//                 .into_iter()
//                 .flatten()
//                 .collect::<Vec<_>>();
//             println!("Calling out to {ext:?} with {args:?}");
//         }
//         None => unreachable!(),
//     }
// }
