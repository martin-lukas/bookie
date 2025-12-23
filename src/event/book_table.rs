use crate::event::Message;
use ratatui::crossterm::event::{self, KeyCode, KeyModifiers};

pub fn handle_key(key: event::KeyEvent) -> Option<Message> {
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
