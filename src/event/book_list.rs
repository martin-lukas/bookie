use std::cmp::max;
use crate::domain::{
    app::{AddBookForm, App, Field},
    view::View,
};
use crossterm::event::{Event, KeyCode};
use log::info;
use crate::domain::app::DEFAULT_RATING;

pub fn handle_event(app: &mut App, event: Event) {
    if let Event::Key(key) = event {
        match key.code {
            KeyCode::Up => {
                info!("Moving up the book list");
                if app.selected > 0 {
                    app.selected -= 1;
                }
            }
            KeyCode::Down => {
                info!("Moving down the book list");
                if app.selected + 1 < app.books.len() {
                    app.selected += 1;
                }
            }
            KeyCode::Enter => {
                info!("Changing view to Book Detail");
                app.view = View::BookDetail;
                app.should_refresh = true;
            }
            KeyCode::Char('a') => {
                info!("Changing view to Add Book");
                app.view = View::AddBook;
                app.add_book_form = Some(AddBookForm {
                    title: String::new(),
                    author: String::new(),
                    year: String::new(),
                    rating: DEFAULT_RATING,
                    active: Field::Title,
                    error: String::new(),
                });
                app.should_refresh = true;
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
