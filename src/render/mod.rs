pub mod book_detail;
pub mod book_form;
pub mod book_list;
mod table;

use crate::domain::layout::Rect;
use crate::domain::{app::App, view::View};
use crossterm::{
    cursor::{Hide, MoveTo, SetCursorStyle},
    execute,
    terminal::{Clear, ClearType},
};
use std::io::{self, stdout, Write};

pub const STAR: &str = "⭑"; // ⭐/ ✰ / ★ / ⭑

pub fn render(app: &App) -> io::Result<()> {
    book_list::render_book_list(&app)?;
    match app.layout.detail.view {
        View::BookDetail => book_detail::render_book_detail(&app)?,
        View::BookForm => book_form::render_add_book(&app)?,
        View::BookList => panic!("Book list view loaded into detail pane."),
    }
    Ok(())
}

pub fn clear_rect(out: &mut impl Write, rect:&Rect) -> io::Result<()> {
    for i in 0..rect.height {
        execute!(
            out,
            MoveTo(rect.x, rect.y + i),
            Clear(ClearType::CurrentLine),
        )?;
    }
    Ok(())
}
