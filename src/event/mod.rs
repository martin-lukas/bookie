use crate::model::model::{Focus, Model};
use ratatui::crossterm::event::{self, Event, KeyCode, KeyModifiers};
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
        Focus::Table => handle_table_key(key),
        Focus::Info => handle_info_key(key),
        Focus::Status => handle_status_key(key),
    }
}

fn handle_table_key(key: event::KeyEvent) -> Option<Message> {
    match (key.code, key.modifiers) {
        (KeyCode::Char('c'), mods) if mods.contains(KeyModifiers::CONTROL) => Some(Message::Quit),
        (KeyCode::Char('q'), _) => Some(Message::Quit),
        (KeyCode::Char('r'), _) => Some(Message::RefreshState),
        (KeyCode::Down, _) => Some(Message::NextBook),
        (KeyCode::Up, _) => Some(Message::PreviousBook),
        (KeyCode::Char('a'), _) => Some(Message::AddBook),
        (KeyCode::Char('e'), _) => Some(Message::EditBook),
        (KeyCode::Char('d'), _) => Some(Message::ConfirmDeleteBook),
        _ => None,
    }
}

fn handle_info_key(key: event::KeyEvent) -> Option<Message> {
    match (key.code, key.modifiers) {
        (KeyCode::Tab, _) | (KeyCode::Down, _) => Some(Message::NextFormField),
        (KeyCode::BackTab, _) | (KeyCode::Up, _) => Some(Message::PreviousFormField),
        (KeyCode::Esc, _) => Some(Message::CancelForm),
        (KeyCode::Char('c'), mods) if mods.contains(KeyModifiers::CONTROL) => {
            Some(Message::CancelForm)
        }
        (KeyCode::Enter, _) => Some(Message::SubmitForm),
        (KeyCode::Char(c), _) => Some(Message::InsertChar(c)),
        (KeyCode::Backspace, _) => Some(Message::DeleteChar),
        _ => None,
    }
}

fn handle_status_key(key: event::KeyEvent) -> Option<Message> {
    match (key.code, key.modifiers) {
        (KeyCode::Char('y'), _) => Some(Message::DeleteBook),
        (KeyCode::Char('n'), _) => Some(Message::CancelConfirm),
        (KeyCode::Char('c'), mods) if mods.contains(KeyModifiers::CONTROL) => {
            Some(Message::CancelConfirm)
        }
        _ => None,
    }
}
