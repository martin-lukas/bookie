use crate::model::{model::Model, status::Mode};
use ratatui::{
    prelude::Rect,
    style::{Color, Style},
    text::Line,
    Frame,
};

pub fn render_status(model: &Model, frame: &mut Frame, area: Rect) {
    let line = match &model.status.mode {
        Mode::Ok => Line::styled(
            "Status: OK".to_string(),
            Style::default().fg(Color::DarkGray),
        ),
        Mode::Error(error) => {
            Line::styled(format!("Error: {}", error), Style::default().fg(Color::Red))
        }
        Mode::ConfirmDeleteBook => Line::raw(format!(
            "Do you really want to delete the book '{}'? [y/n]",
            model.get_selected_book_unsafe().title
        )),
    };
    frame.render_widget(line, area);
}
