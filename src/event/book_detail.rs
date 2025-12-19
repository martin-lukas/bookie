use crate::domain::{model::Model, view::View};
use crossterm::event::{Event, KeyCode, KeyModifiers};

pub fn handle_event(app: &mut Model, event: Event) {
    if let Event::Key(key) = event {
        match (key.code, key.modifiers) {
            (KeyCode::Char('c'), mods) if mods.contains(KeyModifiers::CONTROL) => {
                app.should_quit = true
            }
            (KeyCode::Char('q'), _) => app.should_quit = true,
            _ => {}
        }
    }
}
