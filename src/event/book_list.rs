use crate::domain::{
    model::Model,
    book_form::BookForm,
    view::View,
};
use crossterm::event::{Event, KeyCode, KeyModifiers};
use std::cmp::max;
use crate::domain::layout::Pane;

pub fn handle_event(app: &mut Model, event: Event) {
    if let Event::Key(key) = event {
        match (key.code, key.modifiers) {
            (KeyCode::Char('c'), mods) if mods.contains(KeyModifiers::CONTROL) => {
                app.should_quit = true
            }
            (KeyCode::Char('q'), _) => app.should_quit = true,
            (KeyCode::Up, _) => app.move_selected(-1),
            (KeyCode::Down, _) => app.move_selected(1),
            (KeyCode::Char('a'), _) => {
                app.book_form = BookForm::empty();
                app.change_view(Pane::Bottom, View::AddBookForm);
                app.change_focus(Pane::Bottom);
            },
            (KeyCode::Char('e'), _) => {
                app.change_view(Pane::Bottom, View::EditBookForm);
                app.change_focus(Pane::Bottom);
            },
            (KeyCode::Char('d'), _) => {
                app.books.remove(app.selected);
                app.selected = app.selected.saturating_sub(1);
                app.should_refresh = true;
            }
            _ => {}
        }
    }
}
