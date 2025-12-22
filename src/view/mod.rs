mod content;
mod footer;
mod header;

use crate::{
    model::model::Model,
    view::{content::render_content, footer::render_footer, header::render_header},
};
use ratatui::{prelude::*, Frame};

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
