use std::ops::Index;
use crate::domain::layout::Pane;
use crate::domain::view::View;
use crate::domain::{
    model::Model,
    book::Book,
    book_form::{BookForm, FormAction},
};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use uuid::Uuid;

pub fn handle_event(model: &mut Model, event: Event) {
    let Event::Key(key) = event else { return };
    let next_action = {
        let form = &model.book_form;
        map_event_to_action(key, form)
    };
    handle_next_action(model, next_action);
}

fn map_event_to_action(key: KeyEvent, form: &BookForm) -> FormAction {
    match (key.code, key.modifiers) {
        (KeyCode::Char('c'), mods) if mods.contains(KeyModifiers::CONTROL) => {
            FormAction::BackToList
        }
        (KeyCode::Tab, _) => FormAction::VerticalMove(1),
        (KeyCode::Enter, _) => {
            if let Some(error_message) = form.is_valid() {
                FormAction::Error(format!("❗{}❗", error_message))
            } else {
                FormAction::Submit
            }
        }
        (KeyCode::Backspace, _) => FormAction::RemoveChar,
        (KeyCode::Up, _) => FormAction::VerticalMove(-1),
        (KeyCode::Down, _) => FormAction::VerticalMove(1),
        (KeyCode::Left, _) => FormAction::ChangeRating(-1),
        (KeyCode::Right, _) => FormAction::ChangeRating(1),
        (KeyCode::Char(c), mods) if mods.contains(KeyModifiers::SHIFT) => {
            FormAction::AddChar(c.to_ascii_uppercase())
        }
        (KeyCode::Char(c), _) => FormAction::AddChar(c),
        _ => FormAction::None,
    }
}

fn handle_next_action(model: &mut Model, next_action: FormAction) {
    match next_action {
        FormAction::BackToList => {
            model.change_view(Pane::Bottom, View::BookDetail);
            model.change_focus(Pane::Top);
        }
        FormAction::AddChar(c) => model.book_form.add_active_char(c),
        FormAction::RemoveChar => model.book_form.remove_active_last_char(),
        FormAction::ChangeRating(delta) => model.book_form.change_rating(delta),
        FormAction::VerticalMove(delta) => model.book_form.move_active_field(delta),
        FormAction::Error(error_message) => model.book_form.error = error_message,
        FormAction::Submit => {
            let is_new = model.book_form.id.is_none();
            let book_id: Uuid = if is_new {
                model.add_book(Book::new(&model.book_form.clone()))
            } else {
                model.update_selected_book(&model.book_form.clone())
            };
            model.selected = match model.books.iter().position(|book| book.id == book_id) {
                Some(index) => index,
                None => panic!("Added book was not found!")
            };
            model.change_view(Pane::Bottom, View::BookDetail);
            model.change_focus(Pane::Top);
        }
        FormAction::None => {}
    }
}
