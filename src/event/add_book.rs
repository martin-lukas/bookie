use crate::domain::app::Field;
use crate::domain::{app::App, book::Book, view::View};
use crossterm::event::{Event, KeyCode, KeyModifiers};

pub fn handle_event(app: &mut App, event: Event) {
    if let Event::Key(key) = event {
        let form = app
            .add_book_form
            .as_mut()
            .expect("Add Book view but form data is not initialized.");
        match (key.code, key.modifiers) {
            (KeyCode::Char('c'), mods) if mods.contains(KeyModifiers::CONTROL) => {
                app.should_quit = true
            }
            (KeyCode::Enter, mods) if mods.is_empty() && form.active_field != Field::Rating => {
                form.move_active(1)
            }
            (KeyCode::Enter, mods) if mods.is_empty() => {
                // TODO: Enter goes below line... And last line enter - submit.
                if let Some(error_message) = form.is_valid() {
                    form.error = format!("❗{}❗", error_message);
                } else {
                    let book = Book {
                        title: form.title.to_string(),
                        author: form.author.to_string(),
                        year: form.year.parse::<u16>().unwrap(),
                        rating: form.rating,
                    };
                    app.add_book(book);
                    app.change_view(View::BookList);
                }
            }
            (KeyCode::Backspace, mods) if mods.is_empty() => form.remove_active_last_char(),
            (KeyCode::Up, mods) if mods.is_empty() => form.move_active(-1),
            (KeyCode::Down, mods) if mods.is_empty() => form.move_active(1),
            (KeyCode::Left, mods) if mods.is_empty() => form.change_rating(-1),
            (KeyCode::Right, mods) if mods.is_empty() => form.change_rating(1),
            (KeyCode::Char(c), mods) if !mods.contains(KeyModifiers::CONTROL) => {
                let new_char = if mods.contains(KeyModifiers::SHIFT) {
                    c.to_ascii_uppercase()
                } else {
                    c
                };
                form.add_active_char(new_char)
            }
            _ => {}
        }
    }
}
