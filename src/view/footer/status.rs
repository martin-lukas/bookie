use crate::model::{status::StatusMode, Model};
use ratatui::{
    prelude::Rect,
    style::{Color, Style},
    text::Line,
    widgets::{Block, Padding, Paragraph},
    Frame,
};

pub fn render_status(model: &Model, frame: &mut Frame, area: Rect) {
    let line = match &model.status.mode {
        StatusMode::Ok => Line::styled(
            "Status: OK".to_string(),
            Style::default().fg(Color::DarkGray),
        ),
        StatusMode::Error(error) => {
            Line::styled(format!("Error: {}", error), Style::default().fg(Color::Red))
        }
        StatusMode::ConfirmDeleteBook => match model.get_selected_book() {
            Some(book) => Line::styled(
                format!(
                    "Do you really want to delete the book '{}'? [y/n]",
                    book.title
                ),
                Style::default().fg(Color::LightYellow),
            ),
            None => Line::styled(
                "Book to be deleted was not found",
                Style::default().fg(Color::Red),
            ),
        },
    };
    frame.render_widget(
        Paragraph::new(line).block(Block::default().padding(Padding::horizontal(1))),
        area,
    );
}
