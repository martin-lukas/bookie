use crate::domain::app::App;
use crate::domain::view::View;
use crossterm::event;
use log::info;
use std::io;

mod add_book;
mod book_detail;
mod book_list;

pub fn handle_event(app: &mut App) -> io::Result<()> {
    app.should_refresh = false;
    let event = event::read()?;
    info!("Event registered: {:?}", event);
    match app.active_view {
        View::BookList => book_list::handle_event(app, event),
        View::BookDetail => book_detail::handle_event(app, event),
        View::AddBook => add_book::handle_event(app, event),
    };
    Ok(())
}
