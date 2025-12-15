use crate::domain::app::{AddBookForm, App};
use crossterm::{
    cursor::{MoveToNextLine, Show},
    execute,
    style::Print,
};
use std::io::{self, stdout};

pub fn render_add_book(app: &App) -> io::Result<()> {
    let mut out = stdout();
    execute!(out, Show)?;
    if let Some(AddBookForm {
        title,
        author,
        year,
        active,
    }) = &app.add_book_form
    {
        execute!(out, Print(format!("Title: {}", title)), MoveToNextLine(1))?;
        execute!(out, Print(format!("Author: {}", title)), MoveToNextLine(1))?;
        execute!(out, Print(format!("Year: {}", title)), MoveToNextLine(1))?;
        // execute!(out, Move(0,0))?; //active.index()
    } else {
        panic!("On Add Book view, but the form data is None.");
    }

    Ok(())
}
