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
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, List, ListItem, ListState, Padding},
    Frame, Terminal, TerminalOptions, Viewport,
};

use crate::projects::{self, setup, Project};

struct ProjectList {
    state: ListState,
    items: Vec<Project>,
}

impl ProjectList {
    fn new(items: Vec<Project>) -> Self {
        ProjectList {
            state: ListState::default(),
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

    fn setup_project(&mut self) -> Result<bool, String> {
        let project = match self.state.selected() {
            Some(i) => match self.items.get(i) {
                Some(p) => Ok(Some(p)),
                None => Err("Project index out of bounds"),
            },
            None => Ok(None),
        };
        match project.unwrap() {
            Some(p) => match setup(p) {
                Ok(_) => Ok(true),
                Err(e) => Err(e),
            },
            None => Ok(false),
        }
    }
}

/// This struct holds the current state of the app. In particular, it has the `items` field which is
/// a wrapper around `ListState`. Keeping track of the items state let us render the associated
/// widget with its state and have access to features such as natural scrolling.
pub struct CliApp {
    items: ProjectList,
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

fn ui(f: &mut Frame<CrosstermBackend<Stdout>>, app: &mut CliApp) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([Constraint::Percentage(90), Constraint::Percentage(10)].as_ref())
        .split(f.size());
    let title = "Which project do you want to setup?";
    let padding = Padding::uniform(1);
    let block = Block::default()
        .padding(padding)
        .title_alignment(Alignment::Center)
        .title(Span::styled(
            title,
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Blue),
        ));
    let footer = Block::default();
    let items: Vec<ListItem> = app
        .items
        .items
        .iter()
        .map(|i| ListItem::new(i.name.clone()))
        .collect();
    let list = List::new(items)
        .block(block)
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .fg(Color::Blue)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");
    f.render_stateful_widget(list, chunks[0], &mut app.items.state);
    f.render_widget(footer, chunks[1]);
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
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('j') => app.items.next(),
                        KeyCode::Down => app.items.next(),
                        KeyCode::Char('k') => app.items.previous(),
                        KeyCode::Up => match app.items.setup_project() {
                            Ok(result) => {
                                if result {
                                    break;
                                }
                            }
                            Err(e) => return Err(e),
                        },
                        _ => {}
                    }
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now()
        }
    }
    Ok(())
}
