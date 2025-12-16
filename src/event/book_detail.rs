use crate::domain::{app::App, view::View};
use crossterm::event::{Event, KeyCode};

pub fn handle_event(app: &mut App, event: Event) {
    if let Event::Key(key) = event {
        match key.code {
            KeyCode::Backspace => app.change_view(View::BookList),
            KeyCode::Char('q') => app.should_quit = true,
            _ => {}
        }
    }
}
