use crate::domain::model::Model;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    prelude::{Color, Style},
    widgets::{Block, Borders, Padding, Paragraph},
    Frame,
};
use std::collections::HashSet;

pub fn render_book_stats(model: &Model, frame: &mut Frame, area: Rect) {
    let block = Block::default()
        .title("Stats")
        .padding(Padding::horizontal(1))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    frame.render_widget(&block, area);

    let inner = block.inner(area);

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(0), Constraint::Fill(1)])
        .split(inner);

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
}
