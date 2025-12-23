mod book_info;
mod book_table;
mod status;

use crate::model::model::{Focus, Model};
use ratatui::crossterm::event::{self, Event};
use std::time::Duration;

#[derive(PartialEq)]
pub enum Message {
    Quit,
    RefreshState,
    // Table messages
    NextBook,
    PreviousBook,
    AddBook,
    EditBook,
    DeleteBook,
    ConfirmDeleteBook,
    CancelConfirm,
    // Form messages
    CancelForm,
    InsertChar(char),
    DeleteChar,
    IncreaseRating,
    DecreaseRating,
    NextFormField,
    PreviousFormField,
    SubmitForm,
}

pub fn handle_event(model: &Model) -> color_eyre::Result<Option<Message>> {
    if event::poll(Duration::from_millis(250))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                return Ok(handle_key(model, key));
            }
        }
    }
    Ok(None)
}

fn handle_key(model: &Model, key: event::KeyEvent) -> Option<Message> {
    match model.focus {
        Focus::Table => book_table::handle_key(key),
        Focus::Info => book_info::handle_key(key),
        Focus::Status => status::handle_key(key),
    }
}
