use crate::domain::{model::Model, view::View};
use crossterm::event;
use log::info;
use std::io;
use crate::domain::layout::Pane;

mod book_detail;
mod book_form;
mod book_list;

pub fn handle_event(model: &mut Model) -> io::Result<()> {
    model.should_refresh = false;
    let event = event::read()?;
    info!("Event registered: {:?}", event);
    // TODO: hardcoded views in panes?
    match model.layout.focused {
        Pane::Top =>   book_list::handle_event(model, event),
        Pane::Bottom => {
            match model.view_map[&model.layout.focused] {
                View::BookDetail => book_detail::handle_event(model, event),
                View::AddBookForm | View::EditBookForm => book_form::handle_event(model, event),
                _ => {}
            }
        }
    }

    Ok(())
}
