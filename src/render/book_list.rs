use crate::{
    domain::{app::App, book::STAR},
    util::rpad,
};
use crossterm::{
    cursor::MoveToNextLine,
    execute,
    style::{
        Attribute, Color, Print, PrintStyledContent, ResetColor, SetAttribute, SetForegroundColor,
        Stylize,
    },
};
use std::io::{self, stdout};

const COL_ID: usize = 4;
const COL_TITLE: usize = 30;
const COL_AUTHOR: usize = 20;
const COL_YEAR: usize = 6;
const COL_RATING: usize = 6;
const TABLE_WIDTH: usize = COL_ID + COL_TITLE + COL_AUTHOR + COL_YEAR + COL_RATING;

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
                SetAttribute(Attribute::Bold),
                SetForegroundColor(Color::Yellow),
                Print(rpad(&(i + 1).to_string(), COL_ID)),
                Print(rpad(&book.title, COL_TITLE)),
                Print(rpad(&book.author, COL_AUTHOR)),
                Print(rpad(&book.year.to_string(), COL_YEAR)),
                Print(rpad(&STAR.repeat(book.rating as usize), COL_YEAR)),
                ResetColor,
                SetAttribute(Attribute::Reset),
            )?;
        } else {
            execute!(
                out,
                Print(rpad(&(i + 1).to_string(), COL_ID)),
                Print(rpad(&book.title, COL_TITLE)),
                Print(rpad(&book.author, COL_AUTHOR)),
                Print(rpad(&book.year.to_string(), COL_YEAR)),
                PrintStyledContent(rpad(&STAR.repeat(book.rating as usize), COL_YEAR).yellow()),
            )?;
        }
        execute!(out, MoveToNextLine(1))?;
    }
    Ok(())
}

fn head_row_string() -> String {
    format!(
        "{}{}{}{}{}",
        rpad("#", COL_ID),
        rpad("Title", COL_TITLE),
        rpad("Author", COL_AUTHOR),
        rpad("Year", COL_YEAR),
        rpad("Rating", COL_YEAR),
    )
}

fn separator_row_string() -> String {
    "-".repeat(TABLE_WIDTH)
}
