use crate::{model::Model, view::with_panel};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    widgets::Paragraph,
    Frame,
};

pub fn render_book_stats(model: &Model, frame: &mut Frame, area: Rect) {
    with_panel(frame, area, "Stats", |frame, area| {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(0), Constraint::Fill(1)])
            .split(area);

        frame.render_widget(
            Paragraph::new("Books:\nBooks read:\nAuthors read:\nPages read:"),
            chunks[0],
        );

        frame.render_widget(
            Paragraph::new(format!(
                "{}\n{}\n{}\n{}",
                model.books.len(),
                model.books_read(),
                model.unique_authors_read(),
                model.pages_read(),
            ))
            .alignment(Alignment::Right),
            chunks[1],
        );
    });
}
