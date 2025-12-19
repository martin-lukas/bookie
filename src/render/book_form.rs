use crate::{
    domain::{
        app::App,
        book_form::{BookForm, Field},
    },
    render::STAR,
    util::rpad,
};
use crossterm::{
    cursor::{MoveToColumn, MoveToNextLine, MoveUp, SetCursorStyle, Show},
    execute,
    style::{style, PrintStyledContent, Stylize},
    terminal::{Clear, ClearType},
};
use std::{
    cmp::max,
    io::{self, stdout},
};
use unicode_width::UnicodeWidthStr;

const COL_FIELD: usize = 8;

pub fn render_add_book(app: &App) -> io::Result<()> {
    let mut out = stdout();
    execute!(out, Clear(ClearType::All))?;

    if let Some(BookForm {
        id: _id,
        title,
        author,
        year,
        pages,
        rating,
        note,
        active_field,
        error,
    }) = &app.book_form
    {
        let rows = [
            ("Title:", title.to_string()),
            ("Author:", author.to_string()),
            ("Year:", year.to_string()),
            ("Pages:", pages.to_string()),
            ("Rating:", STAR.repeat(*rating as usize)),
            ("Note:", note.to_string()),
        ];
        for (i, (label, value)) in rows.iter().enumerate() {
            let value_style = if i == Field::Rating.index() {
                value.as_str().yellow()
            } else {
                style(value.as_str())
            };
            execute!(
                out,
                PrintStyledContent(rpad(label, COL_FIELD).bold()),
                PrintStyledContent(value_style),
                MoveToNextLine(1)
            )?;
        }

        if !error.is_empty() {
            execute!(
                out,
                MoveToNextLine(1),
                PrintStyledContent(error.clone().bold().red()),
                MoveUp(1),
            )?;
        }
        let offset_opt = match active_field {
            Field::Title => Some(title.width()),
            Field::Author => Some(author.width()),
            Field::Year => Some(year.width()),
            Field::Pages => Some(pages.width()),
            Field::Rating => Some(max(0, *rating as i8 - 1) as usize),
            Field::Note => Some(note.width()),
        };
        if let Some(offset) = offset_opt {
            if *active_field == Field::Rating {
                execute!(out, SetCursorStyle::BlinkingUnderScore)?;
            }
            execute!(
                out,
                MoveUp((Field::COUNT - active_field.index()) as u16),
                MoveToColumn((COL_FIELD + offset) as u16),
                Show
            )?;
        }
    } else {
        panic!("On Add Book view, but the form data is not initialized.");
    }

    Ok(())
}
