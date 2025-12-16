use crate::{domain::app::App, render::STAR, util::rpad};
use crossterm::{
    cursor::MoveToNextLine,
    execute,
    style::{Print, PrintStyledContent, Stylize},
};
use std::io::{self, stdout};

const COL_FIELD: usize = 8;

pub fn render_book_detail(app: &App) -> io::Result<()> {
    if let Some(book) = app.books.get(app.selected) {
        let mut out = stdout();
        execute!(
            out,
            PrintStyledContent(rpad("Title:", COL_FIELD).bold()),
            Print(&book.title),
            MoveToNextLine(1)
        )?;
        execute!(
            out,
            PrintStyledContent(rpad("Author:", COL_FIELD).bold()),
            Print(&book.author),
            MoveToNextLine(1),
        )?;
        execute!(
            out,
            PrintStyledContent(rpad("Year:", COL_FIELD).bold()),
            Print(book.year),
            MoveToNextLine(1)
        )?;
        execute!(
            out,
            PrintStyledContent(rpad("Rating:", COL_FIELD).bold()),
            PrintStyledContent(STAR.repeat(book.rating as usize).yellow()),
            MoveToNextLine(1)
        )?;

        Ok(())
    } else {
        panic!("Non-existent book selected for rendering!")
    }
}
