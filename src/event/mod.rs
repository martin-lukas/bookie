use crate::domain::model::{Model, RunningState};
use ratatui::crossterm::event::{self, Event, KeyCode, KeyModifiers};
use std::time::Duration;

#[derive(PartialEq)]
pub enum Message {
    Quit,
    NextBook,
    PreviousBook,
    NewBook,
    EditBook,
    DeleteBook,
}

pub fn handle_event(_: &Model) -> color_eyre::Result<Option<Message>> {
    if event::poll(Duration::from_millis(250))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                return Ok(handle_key(key));
            }
        }
    }
    Ok(None)
}

fn handle_key(key: event::KeyEvent) -> Option<Message> {
    match (key.code, key.modifiers) {
        (KeyCode::Char('c'), mods) if mods.contains(KeyModifiers::CONTROL) => Some(Message::Quit),
        (KeyCode::Char('q'), _) => Some(Message::Quit),
        (KeyCode::Down, _) => Some(Message::NextBook),
        (KeyCode::Up, _) => Some(Message::PreviousBook),
        (KeyCode::Char('a'), _) => Some(Message::NewBook),
        (KeyCode::Char('e'), _) => Some(Message::EditBook),
        (KeyCode::Char('d'), _) => Some(Message::DeleteBook),
        _ => None,
    }
}

pub fn update(model: &mut Model, msg: Message) -> Option<Message> {
    match msg {
        Message::Quit => {
            model.running_state = RunningState::Done;
        }
        Message::NextBook => {
            if model.selected + 1 < model.books.len() {
                model.selected += 1;
            }
            model.table_state.select(Some(model.selected));
        }
        Message::PreviousBook => {
            if model.selected > 0 {
                model.selected -= 1;
            }
            model.table_state.select(Some(model.selected));
        }
        Message::NewBook => {}
        Message::EditBook => {}
        Message::DeleteBook => {
            // TODO: before activating - figure out barebones confirmation...
            model.books.remove(model.selected);
            model.selected = model.selected.clamp(0, model.books.len() - 1);
        }
    }
    None
}
