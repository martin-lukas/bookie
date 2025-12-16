use crate::{
    domain::{
        app::{AddBookForm, App, Field},
        book::STAR,
    },
    util::rpad,
};
use crossterm::{
    cursor::{MoveToColumn, MoveToNextLine, MoveUp, Show},
    execute,
    style::{Print, PrintStyledContent, Stylize},
    terminal::{Clear, ClearType},
};
use std::io::{self, stdout};

const COL_FIELD: usize = 8;

pub fn render_add_book(app: &App) -> io::Result<()> {
    let mut out = stdout();
    execute!(out, Clear(ClearType::All))?;

    if let Some(AddBookForm {
        title,
        author,
        year,
        rating,
        active,
        error,
    }) = &app.add_book_form
    {
        execute!(
            out,
            PrintStyledContent(rpad("Title:", COL_FIELD).bold()),
            Print(title),
            MoveToNextLine(1)
        )?;
        execute!(
            out,
            PrintStyledContent(rpad("Author:", COL_FIELD).bold()),
            Print(author),
            MoveToNextLine(1)
        )?;
        execute!(
            out,
            PrintStyledContent(rpad("Year:", COL_FIELD).bold()),
            Print(year),
            MoveToNextLine(1)
        )?;

        execute!(
            out,
            PrintStyledContent(rpad("Rating:", COL_FIELD).bold()),
            PrintStyledContent(STAR.repeat((*rating) as usize).yellow()),
            MoveToNextLine(1)
        )?;

        if !error.is_empty() {
            execute!(
                out,
                MoveToNextLine(1),
                PrintStyledContent(error.clone().bold().red()),
                MoveUp(1),
            )?;
        }
        let offset: i8 = match active {
            Field::Title => title.len() as i8,
            Field::Author => author.len() as i8,
            Field::Year => year.len() as i8,
            Field::Rating => -1,
        };
        if offset != -1 {
            execute!(
                out,
                MoveUp((Field::COUNT - active.index()) as u16),
                MoveToColumn(((COL_FIELD as i8) + offset) as u16),
                Show
            )?;
        }
    } else {
        panic!("On Add Book view, but the form data is not initialized.");
    }

    Ok(())
}
