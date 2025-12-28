use crate::{model::Model, view::with_panel};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    widgets::Paragraph,
    Frame,
};

const LABELS: &[&str] = &[
    "Books in DB:",
    "Authors in DB:",
    "",
    "To read:",
    "Reading:",
    "Read:",
    "",
    "2026",
    "Pages read:",
];

pub fn render_book_stats(model: &Model, frame: &mut Frame, area: Rect) {
    with_panel(frame, area, "Stats", |frame, area| {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(0), Constraint::Fill(1)])
            .split(area);

        frame.render_widget(Paragraph::new(LABELS.join("\n")), chunks[0]);

        frame.render_widget(
            Paragraph::new(format!(
                "{}\n{}\n\n{}\n{}\n{}\n\n\n{}",
                model.books.len(),
                model.unique_authors(),
                model.books_to_read(),
                model.books_reading(),
                model.books_read(),
                model.pages_read_in_year(2026),
            ))
            .alignment(Alignment::Right),
            chunks[1],
        );
    });
}
