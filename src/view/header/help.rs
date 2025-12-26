use ratatui::{
    layout::Rect,
    prelude::{Color, Line, Style, Text},
    widgets::{Block, Padding, Paragraph},
    Frame,
};

const HELP_1: &[&str] = &["A: add", "E: edit", "D: delete"];
const HELP_2: &[&str] = &["←/→: choose item in edit", "↑/↓: diff. field in edit"];
const HELP_3: &[&str] = &[];
const HELP_4: &[&str] = &["F: find"];

pub fn render_help_1(frame: &mut Frame, area: Rect) {
    with_help_panel(frame, area, |frame, inner| {
        let help = Text::from(
            HELP_1
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

pub fn render_help_2(frame: &mut Frame, area: Rect) {
    with_help_panel(frame, area, |frame, inner| {
        let help = Text::from(
            HELP_2
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

pub fn render_help_3(frame: &mut Frame, area: Rect) {
    with_help_panel(frame, area, |frame, inner| {
        let help = Text::from(
            HELP_3
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

pub fn render_help_4(frame: &mut Frame, area: Rect) {
    with_help_panel(frame, area, |frame, inner| {
        let help = Text::from(
            HELP_4
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
