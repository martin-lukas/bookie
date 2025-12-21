use crate::{domain::model::Model, view::STAR};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    prelude::{Color, Line, Style, Text},
    widgets::{Block, Borders, Padding, Paragraph},
    Frame,
};
use unicode_width::UnicodeWidthStr;

const LABELS: &[&str] = &["Title:", "Author(s):", "Year:", "Pages:", "Rating:"];

pub fn render_book_details(model: &Model, frame: &mut Frame, area: Rect) {
    let block = Block::default()
        .title("Details")
        .padding(Padding::horizontal(1))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    frame.render_widget(&block, area);
    let inner = block.inner(area);

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(max_label_width(LABELS) + 1),
            Constraint::Fill(1),
        ])
        .split(inner);

    let book = &model.books[model.selected];

    frame.render_widget(
        Paragraph::new(LABELS.join("\n")),
        chunks[0],
    );

    let details = Paragraph::new(
        Text::from(vec![
            Line::raw(&book.title),
            Line::raw(book.authors.join(", ")),
            Line::raw(book.year.to_string()),
            Line::raw(book.pages.to_string()),
            Line::styled(
                STAR.repeat(book.rating as usize),
                Style::default().fg(Color::LightYellow),
            ),
        ])
        .alignment(Alignment::Left),
    )
    .alignment(Alignment::Left);

    frame.render_widget(details, chunks[1]);
}

fn max_label_width(labels: &[&str]) -> u16 {
    labels.iter().map(|l| l.width() as u16).max().unwrap_or(0)
}
