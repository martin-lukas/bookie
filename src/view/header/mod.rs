mod help;
mod title;

use crate::{
    model::Model,
    view::header::{
        help::{render_help_1, render_help_2, render_help_3, render_help_4},
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
            Constraint::Length(13),
            Constraint::Length(29),
            Constraint::Fill(1),
            Constraint::Length(19),
            Constraint::Length(23),
        ])
        .split(area);

    render_help_1(frame, chunks[0]);
    render_help_2(frame, chunks[1]);
    render_title(frame, chunks[2]);
    render_help_3(frame, chunks[3]);
    render_help_4(frame, chunks[4]);
}
