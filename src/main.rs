mod book;
mod logging;
mod persistance;
mod rendering;
mod view;

use crate::{rendering::Renderer, view::View};
use crossterm::event::{self, Event, KeyCode};
use log::info;
use std::io;

fn main() -> io::Result<()> {
    logging::setup_logger().expect("Failed to setup logger");
    info!("BOOKIE STARTED");

    Renderer::init_screen()?;

    let books = persistance::load_books("books.json")?;

    let mut selected: usize = 0;
    let mut view: View = View::List;
    let mut view_changed = false;

    loop {
        Renderer::render(&view, &books, selected, view_changed)?;
        view_changed = false;

        let event = event::read()?;
        info!("Event registered: {:?}", event);
        if let Event::Key(key_event) = event {
            match view {
                View::List => match key_event.code {
                    KeyCode::Up => {
                        info!("Moving up the book list");
                        if selected > 0 {
                            selected -= 1;
                        }
                    }
                    KeyCode::Down => {
                        info!("Moving down the book list");
                        if selected + 1 < books.len() {
                            selected += 1;
                        }
                    }
                    KeyCode::Enter => {
                        info!("Changing view to Detail");
                        view = View::Detail;
                        view_changed = true;
                    }
                    KeyCode::Char('q') => break,
                    _ => {}
                },
                View::Detail => match key_event.code {
                    KeyCode::Backspace => {
                        info!("Changing view to List");
                        view = View::List;
                        view_changed = true;
                    }
                    KeyCode::Char('q') => break,
                    _ => {}
                },
            }
        }
    }

    Renderer::exit_screen()?;
    info!("BOOKIE EXITING");
    Ok(())
}
