use std::{
    error::Error,
    io::{self, Stdout},
    time::{Duration, Instant},
};

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    backend::CrosstermBackend, widgets::ListState, Frame, Terminal, TerminalOptions, Viewport,
};

use crate::projects::{self, Project};

use super::{normal, setting_up};

pub enum Mode {
    Normal,
    SettingUp,
}

pub struct ProjectList {
    pub state: ListState,
    pub mode: Mode,
    pub items: Vec<Project>,
}

impl ProjectList {
    fn new(items: Vec<Project>) -> Self {
        ProjectList {
            state: ListState::default(),
            mode: Mode::Normal,
            items,
        }
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i))
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i))
    }

    pub fn get_selected(&self) -> Option<&Project> {
        match self.state.selected() {
            Some(i) => match self.items.get(i) {
                Some(p) => Some(p),
                None => None,
            },
            None => None,
        }
    }
}

/// This struct holds the current state of the app. In particular, it has the `items` field which is
/// a wrapper around `ListState`. Keeping track of the items state let us render the associated
/// widget with its state and have access to features such as natural scrolling.
pub struct CliApp {
    pub items: ProjectList,
}

impl CliApp {
    pub fn new() -> Self {
        CliApp {
            items: ProjectList::new(projects::get_all().unwrap()),
        }
    }
}

impl Default for CliApp {
    fn default() -> Self {
        Self::new()
    }
}

pub fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, Box<dyn Error>> {
    let stdout = io::stdout();
    enable_raw_mode()?;
    let terminal = Terminal::with_options(
        CrosstermBackend::new(stdout),
        TerminalOptions {
            viewport: Viewport::Inline(16),
        },
    )?;
    Ok(terminal)
}

pub fn restore_terminal() -> Result<(), Box<dyn Error>> {
    disable_raw_mode()?;
    Ok(())
}

fn ui(frame: &mut Frame<CrosstermBackend<Stdout>>, app: &mut CliApp) {
    match app.items.mode {
        Mode::Normal => normal::ui(frame, app),
        Mode::SettingUp => setting_up::ui(frame, app),
    }
}

pub fn run_app(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    mut app: CliApp,
    tick_rate: Duration,
) -> Result<(), String> {
    let mut last_tick = Instant::now();
    loop {
        terminal
            .draw(|frame| ui(frame, &mut app))
            .expect("Failed to draw in terminal");
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if event::poll(timeout).expect("Failed to poll for application events") {
            if let Event::Key(key) = event::read().expect("Failed to read application event") {
                match app.items.mode {
                    Mode::Normal if key.kind == KeyEventKind::Press => match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('j') => app.items.next(),
                        KeyCode::Down => app.items.next(),
                        KeyCode::Char('k') => app.items.previous(),
                        KeyCode::Up => app.items.previous(),
                        KeyCode::Enter => {
                            if app.items.state.selected().is_some() {
                                app.items.mode = Mode::SettingUp;
                            }
                        }
                        _ => {}
                    },
                    Mode::SettingUp if key.kind == KeyEventKind::Press => {
                        if let KeyCode::Char('q') = key.code {
                            break;
                        }
                    }
                    _ => {}
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now()
        }
    }
    Ok(())
}
