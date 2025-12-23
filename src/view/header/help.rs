use ratatui::{
    layout::Rect,
    prelude::{Color, Line, Style, Text},
    widgets::{Block, Padding, Paragraph},
    Frame,
};
use unicode_width::UnicodeWidthStr;

const HELP_LEFT: &[&str] = &["A -> add", "E -> edit", "D -> delete"];
const HELP_RIGHT: &[&str] = &["F -> find (WIP)", "R -> reload"];

pub fn render_help_left(frame: &mut Frame, area: Rect) {
    let help = Text::from(HELP_LEFT.iter().map(|h| Line::raw(*h)).collect::<Vec<Line>>());
    frame.render_widget(
        Paragraph::new(help)
            .block(Block::default().padding(Padding::horizontal(1)))
            .style(Style::default().fg(Color::DarkGray)),
        area,
    );
}

pub fn render_help_right(frame: &mut Frame, area: Rect) {
    let help = Text::from(HELP_RIGHT.iter().map(|h| Line::raw(*h)).collect::<Vec<Line>>());
    frame.render_widget(
        Paragraph::new(help)
            .block(Block::default().padding(Padding::horizontal(1)))
            .style(Style::default().fg(Color::DarkGray)),
        area,
    );
}
