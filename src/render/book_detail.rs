use crate::{domain::model::Model, render::STAR, util::rpad};
use crossterm::{
    cursor::{MoveDown, MoveTo, MoveToColumn},
    execute,
    style::{Print, PrintStyledContent, Stylize},
};
use std::io::{self, stdout};

const COL_FIELD: usize = 8;

pub fn render(app: &Model) -> io::Result<()> {
    let mut out = stdout();
    let rect = &app.layout.bottom; // TODO: hardcoded?
    let col_start = rect.x;
    let row_start = rect.y;
    rect.clear(&mut out)?;

    execute!(out, MoveTo(col_start, row_start))?;
    if let Some(book) = app.books.get(app.selected) {
        execute!(
            out,
            PrintStyledContent(rpad("Title:", COL_FIELD).bold()),
            Print(&book.title),
            MoveDown(1),
            MoveToColumn(col_start),
        )?;
        execute!(
            out,
            PrintStyledContent(rpad("Author:", COL_FIELD).bold()),
            Print(&book.author),
            MoveDown(1),
            MoveToColumn(col_start),
        )?;
        execute!(
            out,
            PrintStyledContent(rpad("Year:", COL_FIELD).bold()),
            Print(&book.year),
            MoveDown(1),
            MoveToColumn(col_start),
        )?;
        execute!(
            out,
            PrintStyledContent(rpad("Pages:", COL_FIELD).bold()),
            Print(&book.pages),
            MoveDown(1),
            MoveToColumn(col_start),
        )?;
        execute!(
            out,
            PrintStyledContent(rpad("Rating:", COL_FIELD).bold()),
            PrintStyledContent(STAR.repeat(book.rating as usize).yellow()),
            MoveDown(1),
            MoveToColumn(col_start),
        )?;
        execute!(
            out,
            PrintStyledContent(rpad("Note:", COL_FIELD).bold()),
            Print(&book.note),
            MoveDown(1),
            MoveToColumn(col_start),
        )?;

        Ok(())
    } else {
        panic!("Non-existent book selected for rendering!")
    }
}
