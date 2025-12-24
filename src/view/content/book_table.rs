use crate::{
    model::{book::Book, model::Model},
    view::STAR,
};
use ratatui::{
    layout::{Constraint, Rect},
    prelude::{Color, Direction, Layout, Modifier, Style, Text},
    widgets::{Block, Borders, Cell, Padding, Row, Scrollbar, ScrollbarOrientation, Table},
    Frame,
};

pub fn render_book_table(model: &mut Model, frame: &mut Frame, area: Rect) {
    with_book_table_panel(frame, area, "Books", |frame, area| {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Fill(1), Constraint::Length(1)])
            .split(area);

        frame.render_stateful_widget(
            create_book_table(&model.books),
            chunks[0],
            &mut model.book_table.table_state,
        );

        let header_height = 1;
        let scrollbar_area = Rect {
            x: chunks[1].x,
            y: chunks[1].y + header_height,
            width: chunks[1].width,
            height: chunks[1].height.saturating_sub(header_height),
        };

        let scrollbar = Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalRight)
            .style(Style::default().fg(Color::DarkGray))
            .begin_symbol(None)
            .end_symbol(None);

        frame.render_stateful_widget(
            scrollbar,
            scrollbar_area,
            &mut model.book_table.scrollbar_state,
        );
    });
}

fn with_book_table_panel<F>(frame: &mut Frame, area: Rect, title: &str, render: F)
where
    F: FnOnce(&mut Frame, Rect),
{
    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .padding(Padding::new(1, 0, 0, 0))
        .border_style(Style::default().fg(Color::DarkGray));

    frame.render_widget(&block, area);

    let inner = block.inner(area);
    render(frame, inner);
}

fn create_book_table(books: &Vec<Book>) -> Table<'_> {
    let rows = books.iter().map(|b| {
        Row::new(vec![
            Cell::from(b.title.clone()),
            Cell::from(b.authors.join(", ").clone()),
            Cell::from(Text::from(STAR.repeat(b.rating as usize)))
                .style(Style::default().fg(Color::LightYellow)),
        ])
    });

    Table::new(
        rows,
        [
            Constraint::Fill(5),
            Constraint::Fill(4),
            Constraint::Length(7),
        ],
    )
    .header(
        Row::new([
            Cell::from("Title"),
            Cell::from("Authors"),
            Cell::from("Rating"),
        ])
        .style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Cyan),
        ),
    )
    .row_highlight_style(
        Style::default()
            .bg(Color::Black)
            .add_modifier(Modifier::BOLD),
    )
}
