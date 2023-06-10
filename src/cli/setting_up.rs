use std::io::Stdout;

use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Padding},
    Frame,
};

use super::tui::CliApp;

pub fn ui(frame: &mut Frame<CrosstermBackend<Stdout>>, app: &mut CliApp) {
    let project_option = app.items.get_selected();
    match project_option {
        Some(project) => {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints([Constraint::Percentage(90), Constraint::Percentage(10)].as_ref())
                .split(frame.size());
            let title = format!("Setting up \"{}\"", project.name);
            let padding = Padding::uniform(1);
            let block = Block::default()
                .padding(padding)
                .title_alignment(Alignment::Center)
                .title(Span::styled(
                    title,
                    Style::default()
                        .add_modifier(Modifier::BOLD)
                        .fg(Color::Green),
                ));
            let footer = Block::default();
            frame.render_widget(block, chunks[0]);
            frame.render_widget(footer, chunks[1]);
        }
        None => unimplemented!(),
    }
}
