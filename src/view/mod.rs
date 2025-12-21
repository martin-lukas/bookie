mod book_details;
mod book_stats;
mod book_table;
mod title;

use crate::{
    domain::model::Model,
    view::{
        book_details::render_book_details, book_stats::render_book_stats,
        book_table::render_book_table, title::render_title,
    },
};
use ratatui::{prelude::*, Frame};

#[cfg(not(windows))]
pub const STAR: &str = "⭑"; // ⭐/ ✰ / ★ / ⭑
#[cfg(windows)]
pub const STAR: &str = "★"; // ⭐/ ✰ / ★ / ⭑

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
        .constraints([Constraint::Fill(2), Constraint::Fill(3)])
        .split(content_chunks[0]);
    render_title(frame, title_content_chunks[0]);
    render_book_table(model, frame, table_details_chunks[0]);
    render_book_details(model, frame, table_details_chunks[1]);
    render_book_stats(model, frame, content_chunks[1]);
}
