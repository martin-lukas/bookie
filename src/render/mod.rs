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

pub fn render(model: &Model) -> io::Result<()> {
    let mut out = stdout();
    if model.should_refresh {
        model.layout.clear_all(&mut out)?;
    }

    execute!(stdout(), Hide, SetCursorStyle::BlinkingBlock)?;
    model.layout.render_dividers()?;
    render_top(model)?;
    render_bottom(model)?;
    Ok(())
}

fn render_top(model: &Model) -> io::Result<()> {
    // TODO: hardcoded pane-view pairings?
    match model.view_map[&Pane::Top] {
        View::BookList => book_list::render(&model)?,
        _ => {}
    }
    Ok(())
}

fn render_bottom(model: &Model) -> io::Result<()> {
    // TODO: hardcoded pane-view pairings?
    match model.view_map[&Pane::Bottom] {
        View::BookDetail => book_detail::render(&model)?,
        View::AddBookForm | View::EditBookForm => book_form::render(&model)?,
        _ => {}
    }
    Ok(())
}
