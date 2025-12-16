use crate::domain::{app::App, view::View};
use crossterm::event::{Event, KeyCode};
use log::info;

pub fn handle_event(app: &mut App, event: Event) {
    if let Event::Key(key) = event {
        match key.code {
            KeyCode::Backspace => {
                info!("Changing view to Book List");
                app.view = View::BookList;
                app.should_refresh = true;
            }
            KeyCode::Char('q') => app.should_quit = true,
            _ => {}
        }
    }
}
