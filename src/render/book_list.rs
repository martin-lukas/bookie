use crate::domain::book::Book;
use crate::{
    domain::app::App,
    render::{
        table::{Align, Table, TableCell},
        STAR,
    },
};
use crossterm::{cursor::MoveTo, execute, style::Color};
use std::io::{self, stdout};

pub fn render_book_list(app: &App) -> io::Result<()> {
    let mut out = stdout();
    let rect = &app.layout.top; // TODO: Hardcoded?
    execute!(out, MoveTo(rect.x, rect.y))?;
    Table::new(table_header(), table_body(&app.books))
        // TODO: max_col_widths - to limit available space in case of long cells
        .col_widths(vec![4, 45, 39, 4, 5, 6])
        .render(&rect, app.selected)?;
    Ok(())
}

fn table_header() -> Vec<TableCell> {
    vec![
        TableCell::new("#".to_string()).align(Align::Right),
        TableCell::new("Title".to_string()),
        TableCell::new("Author".to_string()),
        TableCell::new("Year".to_string()).align(Align::Right),
        TableCell::new("Pages".to_string()).align(Align::Right),
        TableCell::new("Rating".to_string()),
    ]
}

fn table_body(books: &Vec<Book>) -> Vec<Vec<TableCell>> {
    (0..books.len())
        .map(|i| {
            vec![
                TableCell::new((i + 1).to_string()).align(Align::Right),
                TableCell::new((&books[i]).title.to_string()),
                TableCell::new((&books[i]).author.to_string()),
                TableCell::new((&books[i]).year.to_string()).align(Align::Right),
                TableCell::new((&books[i]).pages.to_string()).align(Align::Right),
                TableCell::new(STAR.repeat((&books[i]).rating as usize)).color(Color::Yellow),
            ]
        })
        .collect()
}
