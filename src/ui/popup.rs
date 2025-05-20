use ratatui::{layout::{Alignment, Constraint}, widgets::{Block, Clear, Paragraph, Wrap}, Frame};

use crate::{app::App, utils::layout::center};

pub fn render_popup(app: &mut App, frame: &mut Frame) {
    let area = center(
        frame.area(),
        Constraint::Percentage(50),
        Constraint::Length(6),
    );
    let selected_count = app
        .files_from
        .items
        .iter()
        .filter(|f| f.is_selected)
        .count();
    let msg = format!(
        "Are you sure you want to move {} file(s)?\nDestination: {} \n\nPress (Y)es to confirm or (N)o/(Esc) to cancel",
        selected_count, app.files_to.path
    );
    let popup = Paragraph::new(msg)
        .wrap(Wrap { trim: true })
        .alignment(Alignment::Center)
        .block(Block::bordered().title("Confirm action"));

    frame.render_widget(Clear, area);
    frame.render_widget(popup, area);
}