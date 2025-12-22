mod book_info;
mod book_stats;
mod book_table;

use crate::{
    model::{book_info::Mode, model::Model},
    view::content::{
        book_info::{render_book_form, render_book_info},
        book_stats::render_book_stats,
        book_table::render_book_table,
    },
};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

pub fn render_content(model: &mut Model, frame: &mut Frame, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Fill(1), Constraint::Length(30)])
        .split(area);
    let table_details_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Fill(2), Constraint::Fill(3)])
        .split(chunks[0]);

    render_book_table(model, frame, table_details_chunks[0]);
    match model.book_info.mode {
        Mode::View => render_book_info(model, frame, table_details_chunks[1]),
        Mode::Add | Mode::Edit => render_book_form(model, frame, table_details_chunks[1]),
    }
    render_book_stats(model, frame, chunks[1]);
}
