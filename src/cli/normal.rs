use std::io::Stdout;

use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, List, ListItem, Padding},
    Frame,
};

use super::tui::CliApp;

pub fn ui(frame: &mut Frame<CrosstermBackend<Stdout>>, app: &mut CliApp) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([Constraint::Percentage(90), Constraint::Percentage(10)].as_ref())
        .split(frame.size());
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
    frame.render_stateful_widget(list, chunks[0], &mut app.items.state);
    frame.render_widget(footer, chunks[1]);
}
