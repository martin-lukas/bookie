use ratatui::{
    layout::{Alignment, Rect},
    prelude::{Color, Line, Modifier, Span, Style, Text},
    widgets::Paragraph,
    Frame,
};

pub fn render_title(frame: &mut Frame, area: Rect) {
    let title = Text::from(vec![
        Line::from(vec![Span::raw(r" __  __  __      __")]),
        Line::from(vec![Span::raw(r"|__)/  \/  \|_/||_ ")]),
        Line::from(vec![Span::raw(r"|__)\__/\__/| \||__")]),
    ]);
    frame.render_widget(
        Paragraph::new(title).alignment(Alignment::Center).style(
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        ),
        area,
    );
}
