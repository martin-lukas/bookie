use crate::domain::view::View;
use crate::domain::{
    app::App,
    book::Book,
    book_form::{BookForm, Field, FormAction},
};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};

pub fn handle_event(app: &mut App, event: Event) {
    let Event::Key(key) = event else { return };
    let next_action = {
        let form = app
            .book_form
            .as_ref()
            .expect("Book form data should have been already defined when on the form page");
        map_event_to_action(key, &form)
    };

    match next_action {
        FormAction::Quit => app.should_quit = true,
        FormAction::AddChar(c) => {
            if let Some(form) = app.book_form.as_mut() {
                form.add_active_char(c);
            }
        }
        FormAction::RemoveChar => {
            if let Some(form) = app.book_form.as_mut() {
                form.remove_active_last_char();
            }
        }
        FormAction::ChangeRating(delta) => {
            if let Some(form) = app.book_form.as_mut() {
                form.change_rating(delta);
            }
        }
        FormAction::VerticalMove(delta) => {
            if let Some(form) = app.book_form.as_mut() {
                form.move_active_field(delta);
            }
        }
        FormAction::Error(error_message) => {
            if let Some(form) = app.book_form.as_mut() {
                form.error = error_message;
            }
        }
        FormAction::Submit => {
            let form = app
                .book_form
                .take()
                .expect("Book form data should have been already defined when on the form page");
            if form.id.is_none() {
                let book = Book::new(&form);
                app.add_book(book);
            } else {
                app.update_selected_book(&form);
            }
            app.change_view(View::BookList);
        }
        FormAction::None => {}
    }
}

fn map_event_to_action(key: KeyEvent, form: &BookForm) -> FormAction {
    match (key.code, key.modifiers) {
        (KeyCode::Char('c'), mods) if mods.contains(KeyModifiers::CONTROL) => FormAction::Quit,
        (KeyCode::Enter, _) if form.active_field != Field::get_last() => {
            FormAction::VerticalMove(1)
        }
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
