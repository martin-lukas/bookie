mod book_detail;
mod book_form;
mod book_list;
mod table;

use crate::domain::{model::Model, layout::Pane, view::View};
use crossterm::{
    cursor::{Hide, SetCursorStyle},
    execute,
};
use std::io::{self, stdout, Write};

pub const STAR: &str = "⭑"; // ⭐/ ✰ / ★ / ⭑

pub fn render(app: &Model) -> io::Result<()> {
    let mut out = stdout();
    if app.should_refresh {
        app.layout.clear_all(&mut out)?;
    }

    execute!(stdout(), Hide, SetCursorStyle::BlinkingBlock)?;
    app.layout.render_dividers()?;
    render_top(app)?;
    render_bottom(app)?;
    Ok(())
}

fn render_top(app: &Model) -> io::Result<()> {
    // TODO: hardcoded pane-view pairings?
    match app.view_map[&Pane::Top] {
        View::BookList => book_list::render(&app)?,
        _ => {}
    }
    Ok(())
}

fn render_bottom(app: &Model) -> io::Result<()> {
    // TODO: hardcoded pane-view pairings?
    match app.view_map[&Pane::Bottom] {
        View::BookDetail => book_detail::render(&app)?,
        View::AddBookForm | View::EditBookForm => book_form::render(&app)?,
        _ => {}
    }
    Ok(())
}
