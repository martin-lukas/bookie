use crate::{domain::model::Model, util::rpad, view::STAR};
use ratatui::{
    layout::Rect,
    prelude::{Color, Line, Span, Style, Text},
    widgets::{Block, Borders, Padding, Paragraph},
    Frame,
};

pub fn render_book_details(model: &Model, frame: &mut Frame, area: Rect) {
    let book = &model.books[model.selected];

    let lines = vec![
        ("Title:", book.title.to_string()),
        ("Author(s):", book.authors.join(", ").to_string()),
        ("Year:", book.year.to_string()),
        ("Pages:", book.pages.to_string()),
        ("Rating:", STAR.repeat(book.rating as usize)),
    ];

    let text = Text::from(
        lines
            .iter()
            .map(|(label, value)| {
                let value_style = if label == &"Rating:" {
                    Style::default().fg(Color::LightYellow)
                } else {
                    Style::default()
                };
                Line::from(vec![
                    Span::raw(rpad(label, 8).to_string()),
                    Span::styled(value, value_style),
                ])
            })
            .collect::<Vec<Line>>(),
    );

    let details = Paragraph::new(text).block(
        Block::default()
            .title("Details")
            .padding(Padding::horizontal(1))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray)),
    );

    frame.render_widget(details, area);
}
