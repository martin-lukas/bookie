use crate::event::Message;
use ratatui::crossterm::event::{self, KeyCode, KeyModifiers};

pub fn handle_key(key: event::KeyEvent) -> Option<Message> {
    match (key.code, key.modifiers) {
        (KeyCode::Char('y'), _) => Some(Message::DeleteBook),
        (KeyCode::Char('n'), _) => Some(Message::CancelConfirm),
        (KeyCode::Char('c'), mods) if mods.contains(KeyModifiers::CONTROL) => {
            Some(Message::CancelConfirm)
        }
        _ => None,
    }
}
