use crate::domain::{
    model::Model,
    book_form::BookForm,
    view::View,
};
use crossterm::event::{Event, KeyCode, KeyModifiers};
use std::cmp::max;
use crate::domain::layout::Pane;

pub fn handle_event(model: &mut Model, event: Event) {
    if let Event::Key(key) = event {
        match (key.code, key.modifiers) {
            (KeyCode::Char('c'), mods) if mods.contains(KeyModifiers::CONTROL) => {
                model.should_quit = true
            }
            (KeyCode::Char('q'), _) => model.should_quit = true,
            (KeyCode::Up, _) => model.move_selected(-1),
            (KeyCode::Down, _) => model.move_selected(1),
            (KeyCode::Char('a'), _) => {
                model.book_form = BookForm::empty();
                model.change_view(Pane::Bottom, View::AddBookForm);
                model.change_focus(Pane::Bottom);
            },
            (KeyCode::Char('e'), _) => {
                model.change_view(Pane::Bottom, View::EditBookForm);
                model.change_focus(Pane::Bottom);
            },
            (KeyCode::Char('d'), _) => {
                model.books.remove(model.selected);
                model.selected = model.selected.saturating_sub(1);
                model.should_refresh = true;
            }
            _ => {}
        }
    }
}
