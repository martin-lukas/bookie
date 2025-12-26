use crate::model::book::ReadingStatus;
use crate::{
    model::model::Model,
    view::{with_panel, STAR},
};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    prelude::{Color, Line, Modifier, Span, Style, Text},
    widgets::Paragraph,
    Frame,
};
use ratatui_image::StatefulImage;
use unicode_width::UnicodeWidthStr;

const LABELS: &[&str] = &[
    " Title: ",
    " Authors: ",
    " Year: ",
    " Pages: ",
    " Status: ",
    " Rating: ",
    " Note: ",
];

pub fn render_book_info(model: &mut Model, frame: &mut Frame, area: Rect) {
    with_panel(frame, area, "Info", |frame, area| {
        let values = model.get_selected_book().map(|book| {
            vec![
                static_line(&book.title),
                static_line(book.authors.join(", ")),
                static_line(book.year.to_string()),
                static_line(book.pages.to_string()),
                reading_status_line(&book.reading_status, true),
                Line::styled(
                    STAR.repeat(book.rating as usize),
                    Style::default().fg(Color::LightYellow),
                ),
                static_line(&book.note),
            ]
        });

        match values {
            Some(values) => {
                render_book_info_content(LABELS, values, model, frame, area);
            }
            None => {
                render_book_info_empty(frame, area);
            }
        }
    });
}

pub fn render_book_form(model: &mut Model, frame: &mut Frame, area: Rect) {
    with_panel(frame, area, "Info", |frame, area| {
        let form = &model.book_info.form;
        let rating_stars = STAR.repeat(form.rating as usize);
        let values = vec![
            input_line(&form.title, form.cursor == 0),
            input_line(&form.authors, form.cursor == 1),
            input_line(&form.year, form.cursor == 2),
            input_line(&form.pages, form.cursor == 3),
            reading_status_line(&form.reading_status, form.cursor == 4),
            input_line(&rating_stars, form.cursor == 5),
            input_line(&form.note, form.cursor == 6),
        ];
        render_book_info_content(LABELS, values, model, frame, area);
    });
}

fn render_book_info_content(
    labels: &[&str],
    values: Vec<Line>,
    model: &mut Model,
    frame: &mut Frame,
    inner: Rect,
) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(20),
            Constraint::Length(max_label_width(labels)),
            Constraint::Fill(1),
        ])
        .split(inner);

    render_book_cover(model, frame, chunks[0]);
    frame.render_widget(
        Paragraph::new(labels.join("\n")).alignment(Alignment::Right),
        chunks[1],
    );
    frame.render_widget(Paragraph::new(Text::from(values)), chunks[2]);
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

fn render_book_cover(model: &mut Model, frame: &mut Frame, area: Rect) {
    if let Some(image_state) = model.book_info.cover_image.as_mut() {
        let padded_area = if area.width > 1 {
            Rect {
                x: area.x,
                y: area.y,
                width: area.width - 1,
                height: area.height,
            }
        } else {
            area
        };

        let image = StatefulImage::default().resize(ratatui_image::Resize::Fit(None));
        frame.render_stateful_widget(image, padded_area, image_state);
    } else {
        // optional placeholder
    }
}

fn max_label_width(labels: &[&str]) -> u16 {
    labels.iter().map(|l| l.width() as u16).max().unwrap_or(0)
}

fn reading_status_line(status: &ReadingStatus, active: bool) -> Line<'static> {
    let selected = Style::default()
        .fg(if active {
            Color::LightYellow
        } else {
            Color::DarkGray
        })
        .add_modifier(Modifier::BOLD);
    let not_selected = Style::default().fg(Color::DarkGray);

    Line::from(vec![
        Span::styled(
            "TO READ",
            if matches!(status, ReadingStatus::ToRead) {
                selected
            } else {
                not_selected
            },
        ),
        Span::raw(" | "),
        Span::styled(
            "READING",
            if matches!(status, ReadingStatus::Reading) {
                selected
            } else {
                not_selected
            },
        ),
        Span::raw(" | "),
        Span::styled(
            "READ",
            if matches!(status, ReadingStatus::Read) {
                selected
            } else {
                not_selected
            },
        ),
    ])
}

fn static_line(text: impl Into<String>) -> Line<'static> {
    input_line(text, false)
}

/// No matter what gets passed (`&str`/`String`/`Cow<str>`), the `Line` will own it.
///
/// `Into<String>` avoids me having to manually clone before calling this method.
fn input_line(text: impl Into<String>, active: bool) -> Line<'static> {
    let text = text.into();
    if active {
        Line::styled(
            format!("{}█", text),
            Style::default().fg(Color::LightYellow),
        )
    } else {
        Line::raw(text)
    }
}
