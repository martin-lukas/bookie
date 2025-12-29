use crate::{
    model::{
        book::reading_status::ReadingStatus,
        book_info::{form_field::FormField, text_input::TextInput, CoverStatus},
        Model,
    },
    view::{content::max_label_width, with_panel, STAR},
};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use ratatui_image::StatefulImage;
use unicode_segmentation::UnicodeSegmentation;

const LABELS: &[&str] = &[
    " Title: ",
    " Authors: ",
    " Year: ",
    " Pages: ",
    " Status: ",
    " Finished on: ",
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
            static_line(&book.title),
            static_line(book.authors.join(", ")),
            static_line(book.year.to_string()),
            static_line(book.pages.to_string()),
            reading_status_line(&book.reading_status, true),
            static_line(
                book.finished_at
                    .last()
                    .map(|d| d.to_string())
                    .unwrap_or("".to_string()),
            ),
            Line::styled(
                STAR.repeat(book.rating as usize),
                Style::default().fg(Color::LightYellow),
            ),
        ];

        let note = book.note.clone();

        render_book_info_content(LABELS, values, Paragraph::new(note), model, frame, area);
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
            render_text_line(&form.finished_at, form.active == FormField::FinishedAt),
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

    frame.render_widget(Paragraph::new(values), value_chunks[0]);
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
    text_with_cursor(&input.text, input.cursor, active)
}

fn render_text_paragraph(input: &TextInput, active: bool) -> Paragraph<'static> {
    Paragraph::new(text_paragraph_with_cursor(
        &input.text,
        input.cursor,
        active,
    ))
}

fn text_with_cursor(text: &str, cursor: usize, active: bool) -> Line<'static> {
    let graphemes: Vec<&str> = UnicodeSegmentation::graphemes(text, true).collect();

    let base_style = if active {
        Style::default().fg(Color::LightYellow)
    } else {
        Style::default()
    };

    let cursor_style = Style::default().bg(Color::LightYellow).fg(Color::Black);

    let mut spans = Vec::with_capacity(graphemes.len() + 1);

    // Highlight the character under the cursor
    for (i, c) in graphemes.iter().enumerate() {
        if active && i == cursor {
            spans.push(Span::styled((*c).to_string(), cursor_style).to_owned());
        } else {
            spans.push(Span::styled((*c).to_string(), base_style).to_owned());
        }
    }

    // Cursor at end of text: render a block cursor
    if active && cursor == graphemes.len() {
        spans.push(Span::styled(" ".to_string(), cursor_style).to_owned());
    }

    Line::from(spans)
}

fn text_paragraph_with_cursor(text: &str, cursor: usize, active: bool) -> Text<'static> {
    let base = if active {
        Style::default().fg(Color::LightYellow)
    } else {
        Style::default()
    };

    let cursor_style = Style::default().bg(Color::LightYellow).fg(Color::Black);

    let graphemes: Vec<&str> = UnicodeSegmentation::graphemes(text, true).collect();

    let mut lines: Vec<Vec<Span>> = vec![Vec::new()];
    let mut idx = 0;

    for g in graphemes {
        if g == "\n" {
            // Cursor at end of this line
            if active && idx == cursor {
                lines
                    .last_mut()
                    .unwrap()
                    .push(Span::styled(" ".to_string(), cursor_style));
            }

            lines.push(Vec::new());
            idx += 1;
            continue;
        }

        let span = if active && idx == cursor {
            Span::styled(g.to_string(), cursor_style)
        } else {
            Span::styled(g.to_string(), base)
        };

        lines.last_mut().unwrap().push(span);
        idx += 1;
    }

    // Cursor at very end
    if active && idx == cursor {
        lines
            .last_mut()
            .unwrap()
            .push(Span::styled(" ".to_string(), cursor_style));
    }

    Text::from(lines.into_iter().map(Line::from).collect::<Vec<_>>())
}

fn static_line(text: impl Into<String>) -> Line<'static> {
    Line::raw(text.into())
}
