use crate::domain::app::App;
use crossterm::{cursor::MoveToNextLine, execute, style::Print};
use std::io::{self, stdout};

pub fn render_book_detail(app: &App) -> io::Result<()> {
    if let Some(book) = app.books.get(app.selected) {
        let mut out = stdout();
        execute!(
            out,
            Print(format!("Title: {}", book.title)),
            MoveToNextLine(1)
        )?;
        execute!(
            out,
            Print(format!("Author: {}", book.author)),
            MoveToNextLine(1),
        )?;
        execute!(
            out,
            Print(format!("Year: {}", book.year)),
            MoveToNextLine(1)
        )?;

        Ok(())
    } else {
        panic!("Non-existent book selected for rendering!")
    }
}
