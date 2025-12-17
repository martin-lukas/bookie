use crate::{
    domain::app::App,
    render::STAR,
    util::{lpad, rpad},
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

const COL_COUNT: usize = 6;
const COL_ID: usize = 4;
const COL_TITLE: usize = 30;
const COL_AUTHOR: usize = 20;
const COL_YEAR: usize = 4;
const COL_PAGES: usize = 5;
const COL_RATING: usize = 6;
const TABLE_WIDTH: usize =
    COL_ID + COL_TITLE + COL_AUTHOR + COL_YEAR + COL_PAGES + COL_RATING + (COL_COUNT - 1);

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
                SetForegroundColor(Color::Yellow)
            )?;
        }
        execute!(
            out,
            Print(lpad(&(i + 1).to_string(), COL_ID) + " "),
            Print(rpad(&book.title, COL_TITLE) + " "),
            Print(rpad(&book.author, COL_AUTHOR) + " "),
            Print(rpad(&book.year.to_string(), COL_YEAR) + " "),
            Print(lpad(&book.pages.to_string(), COL_PAGES) + " "),
            Print(rpad(&STAR.repeat(book.rating as usize), COL_YEAR).yellow())
        )?;
        if i == app.selected {
            execute!(out, ResetColor, SetAttribute(Attribute::Reset))?;
        }
        execute!(out, MoveToNextLine(1))?;
    }
    Ok(())
}

fn head_row_string() -> String {
    format!(
        "{} {} {} {} {} {}",
        lpad("#", COL_ID),
        rpad("Title", COL_TITLE),
        rpad("Author", COL_AUTHOR),
        rpad("Year", COL_YEAR),
        lpad("Pages", COL_PAGES),
        rpad("Rating", COL_YEAR),
    )
}

fn separator_row_string() -> String {
    "-".repeat(TABLE_WIDTH)
}
