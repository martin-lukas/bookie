use crate::{
    model::{
        book::ReadingStatus,
        book_info::{
            form::{FormField, TextInput},
            CoverStatus,
        },
        Model,
    },
    view::{with_panel, STAR},
};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    prelude::{Color, Line, Modifier, Span, Style, Text},
    widgets::{Block, Borders, Paragraph, Wrap},
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
        let Some(book) = model.get_selected_book() else {
            render_book_info_empty(frame, area);
            return;
        };

        let values = vec![
            static_line(&book.title.clone()),
            static_line(book.authors.join(", ")),
            static_line(book.year.to_string()),
            static_line(book.pages.to_string()),
            reading_status_line(&book.reading_status, false),
            Line::styled(
                STAR.repeat(book.rating as usize),
                Style::default().fg(Color::LightYellow),
            ),
        ];

        let note = book.note.clone();

        render_book_info_content(
            LABELS,
            values,
            Paragraph::new(note).wrap(Wrap { trim: false }),
            model,
            frame,
            area,
        );
    });
}

pub fn render_book_form(model: &mut Model, frame: &mut Frame, area: Rect) {
    with_panel(frame, area, "Info", |frame, area| {
        let form = &model.book_info.form;
        let rating_stars = STAR.repeat(form.rating as usize);
        let values = vec![
            render_text_line(&form.title, form.active == FormField::Title),
            render_text_line(&form.authors, form.active == FormField::Authors),
            render_text_line(&form.year, form.active == FormField::Year),
            render_text_line(&form.pages, form.active == FormField::Pages),
            reading_status_line(
                &form.reading_status,
                form.active == FormField::ReadingStatus,
            ),
            render_text_line(
                &TextInput::new(rating_stars),
                form.active == FormField::Rating,
            ),
        ];

        let note = render_text_paragraph(&form.note, form.active == FormField::Note);

        render_book_info_content(LABELS, values, note, model, frame, area);
    });
}

fn render_book_info_content(
    labels: &[&str],
    values: Vec<Line<'static>>,
    note: Paragraph<'static>,
    model: &mut Model,
    frame: &mut Frame,
    inner: Rect,
) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(16),
            Constraint::Length(max_label_width(labels)),
            Constraint::Fill(1),
        ])
        .split(inner);

    render_book_cover(model, frame, chunks[0]);

    frame.render_widget(
        Paragraph::new(labels.join("\n")).alignment(Alignment::Right),
        chunks[1],
    );

    let value_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(values.len() as u16), Constraint::Fill(1)])
        .split(chunks[2]);

    frame.render_widget(Paragraph::new(Text::from(values)), value_chunks[0]);
    frame.render_widget(note, value_chunks[1]);
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

    frame.render_widget(
        Paragraph::new("Press A to add a book")
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::DarkGray)),
        chunks[1],
    );
}

fn render_book_cover(model: &mut Model, frame: &mut Frame, area: Rect) {
    let padded = if area.width > 1 {
        Rect {
            width: area.width - 1,
            ..area
        }
    } else {
        area
    };

    match &mut model.book_info.cover {
        CoverStatus::Ready(image_state) => {
            let image = StatefulImage::default().resize(ratatui_image::Resize::Scale(None));
            frame.render_stateful_widget(image, padded, image_state);
        }
        CoverStatus::Loading => {
            render_cover_placeholder(frame, padded, "LOADING");
        }
        CoverStatus::None => {
            render_cover_placeholder(frame, padded, "NO COVER");
        }
    }
}

fn render_cover_placeholder(frame: &mut Frame, area: Rect, text: &str) {
    let height = area.height.min(10);
    let limited = Rect { height, ..area };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));

    frame.render_widget(&block, limited);

    let inner = block.inner(limited);

    frame.render_widget(
        Paragraph::new(text)
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::DarkGray)),
        inner,
    );
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

    let normal = Style::default().fg(Color::DarkGray);

    Line::from(vec![
        Span::styled(
            "TO READ",
            if matches!(status, ReadingStatus::ToRead) {
                selected
            } else {
                normal
            },
        ),
        Span::raw(" | "),
        Span::styled(
            "READING",
            if matches!(status, ReadingStatus::Reading) {
                selected
            } else {
                normal
            },
        ),
        Span::raw(" | "),
        Span::styled(
            "READ",
            if matches!(status, ReadingStatus::Read) {
                selected
            } else {
                normal
            },
        ),
    ])
}

/* ---------- shared text rendering ---------- */

fn render_text_line(input: &TextInput, active: bool) -> Line<'static> {
    let text = text_with_cursor(&input.text, input.cursor, active);
    Line::from(text.lines.into_iter().next().unwrap_or_default())
}

fn render_text_paragraph(input: &TextInput, active: bool) -> Paragraph<'static> {
    Paragraph::new(text_with_cursor(&input.text, input.cursor, active)).wrap(Wrap { trim: false })
}

fn text_with_cursor(text: &str, cursor: usize, active: bool) -> Text<'static> {
    let chars: Vec<char> = text.chars().collect();
    let cursor = cursor.min(chars.len());

    let before: String = chars[..cursor].iter().collect();
    let after: String = chars[cursor..].iter().collect();

    let mut spans = Vec::new();

    if !before.is_empty() {
        spans.push(if active {
            Span::styled(before, Style::default().fg(Color::LightYellow))
        } else {
            Span::raw(before)
        });
    }

    if active {
        spans.push(Span::styled("█", Style::default().fg(Color::LightYellow)));
    }

    if !after.is_empty() {
        spans.push(if active {
            Span::styled(after, Style::default().fg(Color::LightYellow))
        } else {
            Span::raw(after)
        });
    }

    Text::from(Line::from(spans))
}

fn static_line(text: impl Into<String>) -> Line<'static> {
    Line::raw(text.into())
}
