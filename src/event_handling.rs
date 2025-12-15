use crate::{app::App, view::View};
use crossterm::event::{self, Event, KeyCode};
use log::info;
use std::io;

pub fn handle_event(app: &mut App) -> io::Result<()> {
    app.view_changed = false;
    let event = event::read()?;
    info!("Event registered: {:?}", event);
    app.view_changed = false;
    match app.view {
        View::List => handle_list_event(app, event),
        View::Detail => handle_detail_event(app, event),
    };
    Ok(())
}

fn handle_list_event(app: &mut App, event: Event) {
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
                info!("Changing view to Detail");
                app.view = View::Detail;
                app.view_changed = true;
            }
            KeyCode::Char('q') => app.should_quit = true,
            _ => {}
        }
    }
}

fn handle_detail_event(app: &mut App, event: Event) {
    if let Event::Key(key) = event {
        match key.code {
            KeyCode::Backspace => {
                info!("Changing view to List");
                app.view = View::List;
                app.view_changed = true;
            }
            KeyCode::Char('q') => app.should_quit = true,
            _ => {}
        }
    }
}
