use crate::domain::{model::Model, view::View};
use crossterm::event;
use log::info;
use std::io;
use crate::domain::layout::Pane;

mod book_detail;
mod book_form;
mod book_list;

pub fn handle_event(app: &mut Model) -> io::Result<()> {
    app.should_refresh = false;
    let event = event::read()?;
    info!("Event registered: {:?}", event);
    // TODO: hardcoded views in panes?
    match app.layout.focused {
        Pane::Top =>   book_list::handle_event(app, event),
        Pane::Bottom => {
            match app.view_map[&app.layout.focused] {
                View::BookDetail => book_detail::handle_event(app, event),
                View::AddBookForm | View::EditBookForm => book_form::handle_event(app, event),
                _ => {}
            }
        }
        Pane::Right => {}
    }

    Ok(())
}
