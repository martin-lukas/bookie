use crate::{model::model::Model, view::STAR};
use ratatui::{
    layout::{Constraint, Rect},
    prelude::{Color, Modifier, Style, Text},
    widgets::{Block, Borders, Cell, Padding, Row, Table},
    Frame,
};

pub fn render_book_table(model: &mut Model, frame: &mut Frame, area: Rect) {
    let rows = model.books.iter().map(|b| {
        Row::new(vec![
            Cell::from(b.title.clone()),
            Cell::from(b.authors.join(", ").clone()),
            Cell::from(Text::from(STAR.repeat(b.rating as usize)))
                .style(Style::default().fg(Color::LightYellow)),
        ])
    });

    let table = Table::new(
        rows,
        [
            Constraint::Fill(5),
            Constraint::Fill(4),
            Constraint::Length(6),
        ],
    )
    .header(
        Row::new([
            Cell::from("Title"),
            Cell::from("Author(s)"),
            Cell::from("Rating"),
        ])
        .style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Cyan),
        ),
    )
    .row_highlight_style(
        Style::default()
            .bg(Color::Black)
            .add_modifier(Modifier::BOLD),
    )
    .block(
        Block::default()
            .title("Books")
            .padding(Padding::horizontal(1))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray)),
    );

    frame.render_stateful_widget(table, area, &mut model.book_table.table_state);
}
