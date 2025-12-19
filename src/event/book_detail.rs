use crate::domain::{model::Model, view::View};
use crossterm::event::{Event, KeyCode, KeyModifiers};

pub fn handle_event(model: &mut Model, event: Event) {
    if let Event::Key(key) = event {
        match (key.code, key.modifiers) {
            (KeyCode::Char('c'), mods) if mods.contains(KeyModifiers::CONTROL) => {
                model.should_quit = true
            }
            (KeyCode::Char('q'), _) => model.should_quit = true,
            _ => {}
        }
    }
}
