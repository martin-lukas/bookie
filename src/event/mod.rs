use crate::domain::{app::App, view::View};
use crossterm::event;
use log::info;
use std::io;

mod book_detail;
mod book_form;
mod book_list;

pub fn handle_event(app: &mut App) -> io::Result<()> {
    app.should_refresh = false;
    let event = event::read()?;
    info!("Event registered: {:?}", event);
    if app.layout.list.is_focused {
        book_list::handle_event(app, event);
    } else if app.layout.detail.is_focused {
        match app.layout.detail.view {
            View::BookDetail => book_detail::handle_event(app, event),
            View::BookForm => book_form::handle_event(app, event),
            View::BookList => panic!("Book list in detail view."),
        }
    }

    Ok(())
}
