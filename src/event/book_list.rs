use crate::domain::{
    app::App,
    book_form::{BookForm, Field, DEFAULT_RATING},
    view::View,
};
use crossterm::event::{Event, KeyCode, KeyModifiers};
use std::cmp::max;

pub fn handle_event(app: &mut App, event: Event) {
    if let Event::Key(key) = event {
        match (key.code, key.modifiers) {
            (KeyCode::Char('c'), mods) if mods.contains(KeyModifiers::CONTROL) => {
                app.should_quit = true
            }
            (KeyCode::Char('q'), _) => app.should_quit = true,
            (KeyCode::Up, _) => app.move_selected(-1),
            (KeyCode::Down, _) => app.move_selected(1),
            (KeyCode::Enter, _) => app.change_detail_view(View::BookDetail),
            (KeyCode::Char('a'), _) => {
                app.book_form = Some(BookForm {
                    id: None,
                    title: String::new(),
                    author: String::new(),
                    year: String::new(),
                    pages: String::new(),
                    rating: DEFAULT_RATING,
                    note: String::new(),
                    active_field: Field::Title,
                    error: String::new(),
                });
                app.change_detail_view(View::BookForm);
            }
            (KeyCode::Char('e'), _) => {
                if let Some(book) = app.books.get(app.selected) {
                    app.book_form = Some(BookForm::new(&book));
                    app.change_detail_view(View::BookForm);
                }
            }
            (KeyCode::Char('d'), _) => {
                app.books.remove(app.selected);
                app.selected = max(0, app.selected - 1);
                app.should_refresh = true;
            }
            _ => {}
        }
    }
}
