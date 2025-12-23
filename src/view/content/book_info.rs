use crate::{
    model::model::Model,
    view::{with_panel, STAR},
};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    prelude::{Color, Line, Style, Text},
    widgets::Paragraph,
    Frame,
};
use unicode_width::UnicodeWidthStr;

const LABELS: &[&str] = &[
    "Title: ",
    "Author(s): ",
    "Year: ",
    "Pages: ",
    "Rating: ",
    "Note: ",
];

pub fn render_book_info(model: &Model, frame: &mut Frame, area: Rect) {
    with_panel(frame, area, "Info", |frame, area| {
        if let Some(book) = model.get_selected_book() {
            let values = vec![
                Line::raw(&book.title),
                Line::raw(book.authors.join(", ")),
                Line::raw(book.year.to_string()),
                Line::raw(book.pages.to_string()),
                Line::styled(
                    STAR.repeat(book.rating as usize),
                    Style::default().fg(Color::LightYellow),
                ),
                Line::raw(book.note.to_string()),
            ];

            render_book_info_content(LABELS, values, frame, area);
        } else {
            render_book_info_empty(frame, area);
        }
    });
}

pub fn render_book_form(model: &Model, frame: &mut Frame, area: Rect) {
    with_panel(frame, area, "Info", |frame, area| {
        let form = &model.book_info.form;
        let rating_stars = STAR.repeat(form.rating as usize);
        let values = vec![
            input_line(&form.title, form.cursor == 0),
            input_line(&form.authors, form.cursor == 1),
            input_line(&form.year, form.cursor == 2),
            input_line(&form.pages, form.cursor == 3),
            input_line(&rating_stars, form.cursor == 4),
            input_line(&form.note, form.cursor == 5),
        ];
        render_book_info_content(LABELS, values, frame, area);
    });
}

fn render_book_info_content(labels: &[&str], values: Vec<Line>, frame: &mut Frame, inner: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(max_label_width(labels)),
            Constraint::Fill(1),
        ])
        .split(inner);

    frame.render_widget(
        Paragraph::new(labels.join("\n")).alignment(Alignment::Right),
        chunks[0],
    );

    frame.render_widget(Paragraph::new(Text::from(values)), chunks[1]);
}

fn render_book_info_empty(frame: &mut Frame, inner: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(1),
            Constraint::Fill(1),
        ])
        .split(inner);

    let hint = Paragraph::new("Press A to add a book")
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::DarkGray));

    frame.render_widget(hint, chunks[1]);
}

fn max_label_width(labels: &[&str]) -> u16 {
    labels.iter().map(|l| l.width() as u16).max().unwrap_or(0)
}

fn input_line(text: &str, active: bool) -> Line<'_> {
    if active {
        Line::styled(
            format!("{}█", text),
            Style::default().fg(Color::LightYellow),
        )
    } else {
        Line::raw(text)
    }
}
