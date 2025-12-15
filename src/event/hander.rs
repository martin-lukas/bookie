use crate::domain::{
    app::{AddBookForm, App, Field},
    view::View,
};
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use log::info;
use std::io;

pub fn handle_event(app: &mut App) -> io::Result<()> {
    app.view_changed = false;
    let event = event::read()?;
    info!("Event registered: {:?}", event);
    app.view_changed = false;
    match app.view {
        View::BookList => handle_book_list_event(app, event),
        View::BookDetail => handle_book_detail_event(app, event),
        View::AddBook => handle_add_book_event(app, event),
    };
    Ok(())
}

fn handle_book_list_event(app: &mut App, event: Event) {
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
                app.view_changed = true;
            }
            KeyCode::Char('a') => {
                info!("Changing view to Add Book");
                app.view = View::AddBook;
                app.add_book_form = Some(AddBookForm {
                    title: String::new(),
                    author: String::new(),
                    year: String::new(),
                    active: Field::Title,
                });
                app.view_changed = true;
            }
            KeyCode::Char('q') => app.should_quit = true,
            _ => {}
        }
    }
}

fn handle_book_detail_event(app: &mut App, event: Event) {
    if let Event::Key(key) = event {
        match key.code {
            KeyCode::Backspace => {
                info!("Changing view to Book List");
                app.view = View::BookList;
                app.view_changed = true;
            }
            KeyCode::Char('q') => app.should_quit = true,
            _ => {}
        }
    }
}

fn handle_add_book_event(app: &mut App, event: Event) {
    if let Event::Key(key) = event {
        match (key.code, key.modifiers) {
            (KeyCode::Enter, mods) if mods.contains(KeyModifiers::CONTROL) => {
                if let Some(AddBookForm {
                    title,
                    author,
                    year,
                    active,
                }) = &app.add_book_form
                {
                    info!("Adding book: {} ({}) [{}]", title, author, year);
                }
                info!("Changing view to Book List");
                app.view = View::BookList;
                app.add_book_form = None;
                app.view_changed = true;
            }
            (KeyCode::Char('c'), mods) if mods.contains(KeyModifiers::CONTROL) => {
                app.should_quit = true;
            }
            (KeyCode::Char(c), mods) if mods.is_empty() => {
                // input.push(c);
                // print!("{}", c);
                // out.flush()?;
            }
            (KeyCode::Backspace, mods) if mods.is_empty() => {
                // if input.pop().is_some() {
                //     execute!(out, MoveLeft(1), Clear(ClearType::UntilNewLine))?;
                //     out.flush()?;
                // }
            }
            _ => {}
        }
    }
}
