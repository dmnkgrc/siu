use std::io::{Read, Stdout};

use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Padding, Paragraph},
    Frame,
};

use super::tui::{ProjectSetup, StepCommandState};

pub fn ui(frame: &mut Frame<CrosstermBackend<Stdout>>, project_setup: &mut ProjectSetup) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([Constraint::Percentage(90), Constraint::Percentage(10)].as_ref())
        .split(frame.size());
    let title = format!("Setting up \"{}\"", project_setup.project.name);
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
    let current_step_index = project_setup.current_step;
    let project = project_setup.project.to_owned();
    let current_step = project.steps.into_iter().nth(current_step_index).unwrap();
    let current_step_setup = &mut project_setup.steps[current_step_index];
    let current_cmd = current_step
        .run
        .into_iter()
        .nth(current_step_setup.current_command)
        .unwrap();
    match current_step_setup.current_command_state {
        StepCommandState::Default => {
            let cmd = current_cmd.get_install_cmd();

            project_setup.steps[current_step_index].current_command_process = Some(cmd);
            project_setup.steps[current_step_index].current_command_state =
                StepCommandState::Running;
        }
        StepCommandState::Running => {
            if let Some(cmd) = current_step_setup.current_command_process.as_mut() {
                match cmd.try_wait() {
                    Ok(Some(_)) => {
                        let output = project_setup.append_to_output("");
                        let paragraph = Paragraph::new(output).block(block.to_owned());
                        frame.render_widget(paragraph, chunks[0]);
                    }
                    Ok(None) => {
                        let mut out = String::new();
                        cmd.stderr
                            .take()
                            .expect("Failed to get stderr")
                            .read_to_string(&mut out)
                            .expect("Failed to read stderr");
                        cmd.stdout
                            .take()
                            .expect("Failed to get stdout")
                            .read_to_string(&mut out)
                            .expect("Failed to read stdout");
                        let output = project_setup.append_to_output(&out);
                        let paragraph = Paragraph::new(output).block(block.to_owned());

                        frame.render_widget(paragraph, chunks[0]);
                    }
                    Err(_) => unimplemented!(),
                }
            }
        }
        StepCommandState::Completed => {
            project_setup.current_step += 1;
        }
    }
    frame.render_widget(footer, chunks[1]);
}
