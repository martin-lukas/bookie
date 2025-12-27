use crate::event::Message;
use ratatui::crossterm::event::{self, KeyCode, KeyModifiers};

pub fn handle_key(key: event::KeyEvent) -> Option<Message> {
    match (key.code, key.modifiers) {
        (KeyCode::Tab, _) => Some(Message::NextFormField),
        (KeyCode::BackTab, _) => Some(Message::PreviousFormField),
        (KeyCode::Esc, _) => Some(Message::CancelForm),
        (KeyCode::Char('c'), mods) if mods.contains(KeyModifiers::CONTROL) => {
            Some(Message::CancelForm)
        }
        (KeyCode::Enter, _) => Some(Message::NewLineChar),
        (KeyCode::Char('s'), mods) if mods.contains(KeyModifiers::CONTROL) => {
            Some(Message::SubmitForm)
        }
        (KeyCode::Char(c), _) => Some(Message::InsertChar(c)),
        (KeyCode::Left, _) => Some(Message::FormLeft),
        (KeyCode::Right, _) => Some(Message::FormRight),
        (KeyCode::Backspace, _) => Some(Message::DeleteChar),
        _ => None,
    }
}
