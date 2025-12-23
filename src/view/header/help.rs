use ratatui::{
    layout::Rect,
    prelude::{Color, Line, Style, Text},
    widgets::{Block, Padding, Paragraph},
    Frame,
};

const HELP_LEFT: &[&str] = &["A -> add", "E -> edit", "D -> delete"];
const HELP_RIGHT: &[&str] = &["F -> find (WIP)", "R -> reload"];

pub fn render_help_left(frame: &mut Frame, area: Rect) {
    with_help_panel(frame, area, |frame, inner| {
        let help = Text::from(
            HELP_LEFT
                .iter()
                .map(|h| Line::raw(*h))
                .collect::<Vec<Line>>(),
        );
        frame.render_widget(
            Paragraph::new(help).style(Style::default().fg(Color::DarkGray)),
            inner,
        );
    });
}

pub fn render_help_right(frame: &mut Frame, area: Rect) {
    with_help_panel(frame, area, |frame, inner| {
        let help = Text::from(
            HELP_RIGHT
                .iter()
                .map(|h| Line::raw(*h))
                .collect::<Vec<Line>>(),
        );
        frame.render_widget(
            Paragraph::new(help).style(Style::default().fg(Color::DarkGray)),
            inner,
        );
    });
}

fn with_help_panel<F>(frame: &mut Frame, area: Rect, render: F)
where
    F: FnOnce(&mut Frame, Rect),
{
    let block = Block::default().padding(Padding::horizontal(1));
    frame.render_widget(&block, area);
    let inner = block.inner(area);
    render(frame, inner);
}
