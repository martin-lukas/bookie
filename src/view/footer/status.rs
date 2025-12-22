use crate::model::model::Model;
use ratatui::{
    prelude::Rect,
    style::{Color, Style},
    text::Line,
    Frame,
};

pub fn render_status(model: &Model, frame: &mut Frame, area: Rect) {
    let content: Line = match &model.book_info.form.error {
        Some(error) => Line::styled(format!("Error: {}", error), Style::default().fg(Color::Red)),
        None => Line::styled(
            "Status: OK".to_string(),
            Style::default().fg(Color::DarkGray),
        ),
    };
    frame.render_widget(content, area);
}
