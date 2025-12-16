pub mod add_book;
pub mod book_detail;
pub mod book_list;

use crate::domain::{app::App, view::View};
use crossterm::{
    cursor::{Hide, MoveTo},
    execute,
    terminal::{Clear, ClearType},
};
use std::io::{self, stdout};

pub fn render(app: &App) -> io::Result<()> {
    reset_screen(app.should_refresh)?;
    match app.active_view {
        View::BookList => book_list::render_book_list(&app)?,
        View::BookDetail => book_detail::render_book_detail(&app)?,
        View::AddBook => add_book::render_add_book(&app)?,
    }
    Ok(())
}

fn reset_screen(should_refresh: bool) -> io::Result<()> {
    let mut out = stdout();
    if should_refresh {
        execute!(out, Clear(ClearType::All))?;
    }
    execute!(out, MoveTo(0, 0), Hide)
}
