use crate::{model::model::Model, view::with_panel};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    widgets::Paragraph,
    Frame,
};
use std::collections::HashSet;

pub fn render_book_stats(model: &Model, frame: &mut Frame, area: Rect) {
    with_panel(frame, area, "Stats", |frame, area| {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(0), Constraint::Fill(1)])
            .split(area);

        frame.render_widget(Paragraph::new("Books:\nAuthors:\nPages:"), chunks[0]);

        frame.render_widget(
            Paragraph::new(format!(
                "{}\n{}\n{}",
                model.books.len(),
                model
                    .books
                    .iter()
                    .flat_map(|b| b.authors.clone())
                    .collect::<HashSet<String>>()
                    .len(),
                model.books.iter().map(|b| b.pages).sum::<u16>(),
            ))
            .alignment(Alignment::Right),
            chunks[1],
        );
    });
}
