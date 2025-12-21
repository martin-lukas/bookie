use crate::domain::model::Model;
use color_eyre::owo_colors::OwoColorize;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
    Frame,
};

pub const STAR: &str = "⭑"; // ⭐/ ✰ / ★ / ⭑

pub fn view(model: &mut Model, frame: &mut Frame) {
    let title_content_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Fill(1)])
        .split(frame.area());
    let content_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Fill(1), Constraint::Length(30)])
        .split(title_content_chunks[1]);
    let table_details_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(content_chunks[0]);
    render_title(model, frame, title_content_chunks[0]);
    render_book_table(model, frame, table_details_chunks[0]);
    render_book_details(model, frame, table_details_chunks[1]);
    render_book_stats(model, frame, content_chunks[1]);
}

fn render_title(model: &mut Model, frame: &mut Frame, area: Rect) {
    let title = Text::from(vec![
        Line::from(vec![Span::raw(".__  __  __ . ...__")]),
        Line::from(vec![Span::raw("|__)/  \\/  \\|_/||_ ")]),
        Line::from(vec![Span::raw("|__)\\__/\\__/| \\||__")]),
    ]);
    frame.render_widget(
        Paragraph::new(title).alignment(Alignment::Center).style(
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        ),
        area,
    );
}

fn render_book_table(model: &mut Model, frame: &mut Frame, area: Rect) {
    let rows = model.books.iter().enumerate().map(|(i, b)| {
        Row::new(vec![
            Cell::from(Text::from(format!("{}", i + 1)).alignment(Alignment::Right)),
            Cell::from(b.title.clone()),
            Cell::from(b.author.clone()),
            Cell::from(Text::from(b.year.to_string()).alignment(Alignment::Right)),
            Cell::from(Text::from(STAR.repeat(b.rating as usize)))
                .style(Style::default().fg(Color::LightYellow)),
        ])
    });

    let table = Table::new(
        rows,
        [
            Constraint::Length(4),
            Constraint::Fill(10),
            Constraint::Fill(8),
            Constraint::Length(4),
            Constraint::Length(7),
        ],
    )
    .header(
        Row::new([
            Cell::from(Text::from("#").alignment(Alignment::Right)),
            Cell::from("Title"),
            Cell::from("Author"),
            Cell::from(Text::from("Year").alignment(Alignment::Right)),
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
    .block(Block::default().title("Books").borders(Borders::ALL));

    frame.render_stateful_widget(table, area, &mut model.table_state);
}

fn render_book_details(model: &Model, frame: &mut Frame, area: Rect) {
    let book = &model.books[model.selected];

    let text = Text::from(vec![
        Line::from(vec![Span::raw("Title: "), Span::raw(&book.title)]),
        Line::from(vec![Span::raw("Author: "), Span::raw(&book.author)]),
        Line::from(vec![Span::raw("Year: "), Span::raw(book.year.to_string())]),
        Line::from(vec![Span::raw("Pages: "), Span::raw(book.pages.to_string())]),
        Line::from(vec![
            Span::raw("Rating: "),
            Span::styled(
                STAR.repeat(book.rating as usize),
                Style::default().fg(Color::LightYellow),
            ),
        ]),
    ]);

    let details =
        Paragraph::new(text).block(Block::default().title("Details").borders(Borders::ALL));

    frame.render_widget(details, area);
}

fn render_book_stats(model: &Model, frame: &mut Frame, area: Rect) {
    frame.render_widget(
        Paragraph::new("Books: 2").block(Block::default().title("Stats").borders(Borders::ALL)),
        area,
    );
}
