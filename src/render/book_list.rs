use crate::{
    domain::{app::App, book::Book},
    util::rpad,
};

use crossterm::{
    cursor::MoveToNextLine,
    execute,
    style::{Print, PrintStyledContent, Stylize},
};
use std::io::{self, stdout};

const COL_ID: usize = 4;
const COL_TITLE: usize = 30;
const COL_AUTHOR: usize = 20;
const COL_YEAR: usize = 4;
const TABLE_WIDTH: usize = COL_ID + COL_TITLE + COL_AUTHOR + COL_YEAR;

pub fn render_book_list(app: &App) -> io::Result<()> {
    let mut out = stdout();
    execute!(
        out,
        PrintStyledContent(head_row_string().bold()),
        MoveToNextLine(1),
        Print(separator_row_string()),
        MoveToNextLine(1),
    )?;

    for (i, book) in app.books.iter().enumerate() {
        if i == app.selected {
            execute!(
                out,
                PrintStyledContent(table_row_string(&book, i).bold().yellow())
            )?;
        } else {
            execute!(out, Print(table_row_string(&book, i)))?;
        }
        execute!(out, MoveToNextLine(1))?;
    }
    Ok(())
}

fn head_row_string() -> String {
    format!(
        "{}{}{}{}",
        rpad("#", COL_ID),
        rpad("Title", COL_TITLE),
        rpad("Author", COL_AUTHOR),
        rpad("Year", COL_YEAR),
    )
}

fn separator_row_string() -> String {
    "-".repeat(TABLE_WIDTH)
}

fn table_row_string(book: &Book, index: usize) -> String {
    format!(
        "{}{}{}{}",
        rpad(&(index + 1).to_string(), COL_ID),
        rpad(&book.title, COL_TITLE),
        rpad(&book.author, COL_AUTHOR),
        rpad(&book.year.to_string(), COL_YEAR),
    )
}
