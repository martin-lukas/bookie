mod help;
mod title;

use crate::{
    model::model::Model,
    view::header::{
        help::{render_help_left, render_help_right},
        title::render_title,
    },
};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

pub fn render_header(_: &Model, frame: &mut Frame, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(17),
            Constraint::Fill(1),
            Constraint::Length(17),
        ])
        .split(area);

    render_help_left(frame, chunks[0]);
    render_title(frame, chunks[1]);
    render_help_right(frame, chunks[2]);
}
