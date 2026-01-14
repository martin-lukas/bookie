pub mod app_event;
mod book_info;
mod book_table;
mod status;

use crate::{
    event::app_event::AppEvent,
    model::{focus::Focus, Model},
};
use ratatui::crossterm::event::{self, Event};
use std::sync::mpsc::Sender;

#[derive(PartialEq)]
pub enum Message {
    Quit,
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
    FormRight,
    FormLeft,
    NextFormField,
    PreviousFormField,
    SubmitForm,
}

pub fn spawn_input_thread(tx: Sender<AppEvent>) {
    std::thread::spawn(move || loop {
        if let Ok(ev) = event::read() {
            match ev {
                Event::Key(key_event) if key_event.kind == event::KeyEventKind::Press => {
                    tx.send(AppEvent::Key(key_event)).ok();
                }
                Event::Resize(_, _) => {
                    tx.send(AppEvent::Resize).ok();
                }
                _ => {}
            }
        }
    });
}

pub fn handle_key(model: &Model, key: event::KeyEvent) -> Option<Message> {
    match model.focus {
        Focus::Table => book_table::handle_key(key),
        Focus::Info => book_info::handle_key(key),
        Focus::Status => status::handle_key(key),
    }
}
