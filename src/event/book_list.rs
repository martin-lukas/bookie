use std::cmp::max;
use crate::domain::{
    app::{AddBookForm, App, Field},
    view::View,
};
use crossterm::event::{Event, KeyCode};
use crate::domain::app::DEFAULT_RATING;

pub fn handle_event(app: &mut App, event: Event) {
    if let Event::Key(key) = event {
        match key.code {
            KeyCode::Up => app.move_selected(-1),
            KeyCode::Down => app.move_selected(1),
            KeyCode::Enter => app.change_view(View::BookDetail),
            KeyCode::Char('a') => {
                app.add_book_form = Some(AddBookForm {
                    title: String::new(),
                    author: String::new(),
                    year: String::new(),
                    rating: DEFAULT_RATING,
                    active_field: Field::Title,
                    error: String::new(),
                });
                app.change_view(View::AddBook);
            }
            KeyCode::Char('d') => {
                app.books.remove(app.selected);
                app.selected = max(0, app.selected - 1);
                app.should_refresh = true;
            }
            KeyCode::Char('q') => app.should_quit = true,
            _ => {}
        }
    }
}
