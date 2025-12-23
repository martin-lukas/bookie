mod content;
mod footer;
mod header;

use crate::{
    model::model::Model,
    view::{content::render_content, footer::render_footer, header::render_header},
};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Padding},
    Frame,
};

#[cfg(not(windows))]
pub const STAR: &str = "⭑"; // ⭐/ ✰ / ★ / ⭑
#[cfg(windows)]
pub const STAR: &str = "★"; // ⭐/ ✰ / ★ / ⭑

pub fn view(model: &mut Model, frame: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .split(frame.area());

    render_header(model, frame, chunks[0]);
    render_content(model, frame, chunks[1]);
    render_footer(model, frame, chunks[2]);
}

pub fn with_panel<F>(frame: &mut Frame, area: Rect, title: &str, render: F)
where
    F: FnOnce(&mut Frame, Rect),
{
    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .padding(Padding::horizontal(1))
        .border_style(Style::default().fg(Color::DarkGray));

    frame.render_widget(&block, area);

    let inner = block.inner(area);
    render(frame, inner);
}
