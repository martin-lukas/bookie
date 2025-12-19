pub mod book_detail;
pub mod book_form;
pub mod book_list;
mod table;

use crate::domain::layout::Pane;
use crate::domain::{app::App, view::View};
use crossterm::{
    cursor::{Hide, SetCursorStyle},
    execute,
};
use std::io::{self, stdout, Write};

pub const STAR: &str = "⭑"; // ⭐/ ✰ / ★ / ⭑

pub fn render(app: &App) -> io::Result<()> {
    let mut out = stdout();
    if app.should_refresh {
        app.layout.clear_all(&mut out)?;
    }

    // TODO: hardcoded pane-view pairings?
    execute!(stdout(), Hide, SetCursorStyle::BlinkingBlock)?;
    match app.view_map[&Pane::Top] {
        View::BookList => book_list::render_book_list(&app)?,
        _ => {}
    }
    match app.view_map[&Pane::Bottom] {
        View::BookDetail => book_detail::render_book_detail(&app)?,
        View::AddBookForm | View::EditBookForm => book_form::render_book_form(&app)?,
        _ => {}
    }
    Ok(())
}
