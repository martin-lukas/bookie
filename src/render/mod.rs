mod book_detail;
mod book_form;
mod book_list;
mod book_stats;
mod table;

use crate::domain::{app::App, layout::Pane, view::View};
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

    execute!(stdout(), Hide, SetCursorStyle::BlinkingBlock)?;
    render_top(app)?;
    render_bottom(app)?;
    render_right(app)?;
    app.layout.render_dividers()?;
    Ok(())
}

fn render_top(app: &App) -> io::Result<()> {
    // TODO: hardcoded pane-view pairings?
    match app.view_map[&Pane::Top] {
        View::BookList => book_list::render(&app)?,
        _ => {}
    }
    Ok(())
}

fn render_bottom(app: &App) -> io::Result<()> {
    // TODO: hardcoded pane-view pairings?
    match app.view_map[&Pane::Bottom] {
        View::BookDetail => book_detail::render(&app)?,
        View::AddBookForm | View::EditBookForm => book_form::render(&app)?,
        _ => {}
    }
    Ok(())
}

fn render_right(app: &App) -> io::Result<()> {
    // TODO: hardcoded pane-view pairings?
    match app.view_map[&Pane::Right] {
        View::BookStats => book_stats::render(&app)?,
        _ => {}
    }
    Ok(())
}
