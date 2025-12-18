use crate::{
    domain::app::App,
    render::{
        table::{render_table, Align, Table, TableCell},
        STAR,
    },
};
use crossterm::{cursor::MoveTo, execute};
use std::io::{self, stdout};

pub fn render_book_list(app: &App) -> io::Result<()> {
    let mut out = stdout();
    let pane = &app.layout.list;
    let col_start = pane.area.x;
    let row_start = pane.area.y;
    execute!(out, MoveTo(col_start, row_start))?;
    render_table(
        &Table::new(
            vec![
                TableCell::new("#".to_string()).align(Align::Right),
                TableCell::new("Title".to_string()),
                TableCell::new("Author".to_string()),
                TableCell::new("Year".to_string()),
                TableCell::new("Pages".to_string()).align(Align::Right),
                TableCell::new("Rating".to_string()),
            ],
            (0..app.books.len())
                .map(|i| {
                    let book = &app.books[i];
                    vec![
                        TableCell::new((i + 1).to_string()).align(Align::Right),
                        TableCell::new(book.title.to_string()),
                        TableCell::new(book.author.to_string()),
                        TableCell::new(book.year.to_string()),
                        TableCell::new(book.pages.to_string()),
                        TableCell::new(STAR.repeat(book.rating as usize)), // TODO: add style to cells
                    ]
                })
                .collect(),
        )
        .col_widths(vec![4, 30, 20, 4, 5, 6]) // TODO: change to max_... to make more flexible
        .sep_width(1),
        app.selected,
    )?;
    Ok(())
}
