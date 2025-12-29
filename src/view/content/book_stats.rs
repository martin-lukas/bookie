use crate::{model::Model, view::with_panel};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    widgets::Paragraph,
    Frame,
};
use unicode_width::UnicodeWidthStr;

pub fn render_book_stats(model: &Model, frame: &mut Frame, area: Rect) {
    with_panel(frame, area, "Stats", |frame, area| {
        let rows = build_stats(model, 2026);

        let labels: Vec<String> = rows.iter().map(|r| r.label.to_string()).collect();
        let values: Vec<String> = rows
            .iter()
            .map(|r| r.value.clone().unwrap_or_default())
            .collect();

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(max_label_width(&labels)),
                Constraint::Fill(1),
            ])
            .split(area);

        frame.render_widget(Paragraph::new(labels.join("\n")), chunks[0]);

        frame.render_widget(
            Paragraph::new(values.join("\n")).alignment(Alignment::Right),
            chunks[1],
        );
    });
}

fn build_stats(model: &Model, year: i32) -> Vec<StatLine> {
    vec![
        StatLine::header("Global:"),
        StatLine::xxxnew("├ Books in DB:", model.books.len().to_string()),
        StatLine::xxxnew("├ Authors in DB:", model.unique_authors().to_string()),
        StatLine::header("└ Status:"),
        StatLine::xxxnew("  ├ Read:", model.books_read().to_string()),
        StatLine::xxxnew("  ├ Reading:", model.books_reading().to_string()),
        StatLine::xxxnew("  └ To read:", model.books_to_read().to_string()),
        StatLine::empty(),
        StatLine::header(format!("{}:", year)),
        StatLine::xxxnew(
            "├ Books:",
            model.books_read_in_year(year as u16).to_string(),
        ),
        StatLine::xxxnew(
            "└ Pages:",
            model.pages_read_in_year(year as u16).to_string(),
        ),
    ]
}

struct StatLine {
    label: String,
    value: Option<String>,
}

impl StatLine {
    fn xxxnew(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: Some(value.into()),
        }
    }

    fn header(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: None,
        }
    }

    fn empty() -> Self {
        Self {
            label: String::new(),
            value: None,
        }
    }
}

fn max_label_width(labels: &Vec<String>) -> u16 {
    labels.iter().map(|l| l.width() as u16).max().unwrap_or(0)
}
