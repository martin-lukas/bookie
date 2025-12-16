use crate::domain::app::MAX_RATING;
use crate::domain::{app::App, app::Field, book::Book, view::View};
use crossterm::event::{Event, KeyCode, KeyModifiers};
use log::info;
use std::cmp::{max, min};

pub fn handle_event(app: &mut App, event: Event) {
    if let Event::Key(key) = event {
        match (key.code, key.modifiers) {
            (KeyCode::Enter, mods) if mods.is_empty() => match &mut app.add_book_form {
                Some(form) => {
                    if form.is_valid() {
                        let book = Book {
                            title: form.title.to_string(),
                            author: form.author.to_string(),
                            year: form.year.parse::<u16>().unwrap(),
                            rating: form.rating,
                        };
                        info!("Adding book: {:?}", book);
                        app.add_book(book);

                        info!("Changing view to Book List");
                        app.view = View::BookList;
                        app.add_book_form = None;
                        app.should_refresh = true;
                    } else {
                        form.error = "Form is not valid".to_string();
                    }
                }
                None => panic!("Add Book view but form data is not initialized."),
            },
            (KeyCode::Char('c'), mods) if mods.contains(KeyModifiers::CONTROL) => {
                app.should_quit = true;
            }
            (KeyCode::Backspace, mods) if mods.is_empty() => match &mut app.add_book_form {
                Some(form) => match form.active {
                    Field::Title => {
                        form.title.pop();
                    }
                    Field::Author => {
                        form.author.pop();
                    }
                    Field::Year => {
                        form.year.pop();
                    }
                    Field::Rating => {
                        form.rating -= 1;
                    }
                },
                None => panic!("Add Book view but form data is not initialized."),
            },
            (KeyCode::Up, mods) if mods.is_empty() => match &mut app.add_book_form {
                Some(form) => {
                    form.error = String::new();
                    form.active = Field::get_by_index(max(0, form.active.index() - 1));
                }
                None => panic!("Add Book view but form data is not initialized."),
            },
            (KeyCode::Down, mods) if mods.is_empty() => match &mut app.add_book_form {
                Some(form) => {
                    form.error = String::new();
                    form.active =
                        Field::get_by_index(min(Field::COUNT - 1, form.active.index() + 1));
                }
                None => panic!("Add Book view but form data is not initialized."),
            },
            (KeyCode::Left, mods) if mods.is_empty() => match &mut app.add_book_form {
                Some(form) => {
                    form.error = String::new();
                    form.rating = max(0, form.rating - 1);
                }
                None => panic!("Add Book view but form data is not initialized."),
            },
            (KeyCode::Right, mods) if mods.is_empty() => match &mut app.add_book_form {
                Some(form) => {
                    form.error = String::new();
                    form.rating = min(MAX_RATING, form.rating + 1);
                }
                None => panic!("Add Book view but form data is not initialized."),
            },
            (KeyCode::Char(c), mods) if mods.is_empty() => match &mut app.add_book_form {
                Some(form) => {
                    form.error = String::new();
                    match form.active {
                        Field::Title => form.title.push(c),
                        Field::Author => form.author.push(c),
                        Field::Year => form.year.push(c),
                        Field::Rating => {}
                    }
                }
                None => panic!("Add Book view but form data is not initialized."),
            },
            (KeyCode::Char(c), mods) if mods.contains(KeyModifiers::SHIFT) => {
                match &mut app.add_book_form {
                    Some(form) => {
                        form.error = String::new();
                        match form.active {
                            Field::Title => form.title.push(c.to_ascii_uppercase()),
                            Field::Author => form.author.push(c.to_ascii_uppercase()),
                            Field::Year => form.year.push(c.to_ascii_uppercase()),
                            Field::Rating => {}
                        }
                    }
                    None => panic!("Add Book view but form data is not initialized."),
                }
            }
            _ => {}
        }
    }
}
