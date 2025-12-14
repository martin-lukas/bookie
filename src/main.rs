mod book;
mod event_handling;
mod logging;
mod persistance;
mod renderer;
mod view;

use crate::{
    event_handling::{EventHandler, HandleResult},
    renderer::Renderer,
    view::View,
};
use crossterm::{
    cursor, event,
    event::{Event, KeyCode},
    execute, terminal,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use log::info;
use std::io::{self, stdout};

fn main() -> io::Result<()> {
    logging::setup_logger().expect("Failed to setup logger");
    info!("BOOKIE STARTED");

    init_screen()?;

    let books = persistance::load_books("books.json")?;

    let mut view = View::new(&books);

    loop {
        Renderer::draw(&view)?;
        let event = event::read()?;
        info!("Event registered: {:?}", event);
        if let Event::Key(key_event) = event {
            match view.handle(&key_event) {
                HandleResult::Handled => {}
                HandleResult::Ignored => match event {
                    Event::Key(key_event) => match key_event.code {
                        KeyCode::Char('q') => break,
                        _ => {}
                    },
                    _ => {}
                },
                HandleResult::Quit => break,
            }
        }
    }

    exit_screen()?;
    info!("BOOKIE EXITING");
    Ok(())
}

fn init_screen() -> io::Result<()> {
    let mut out = stdout();
    terminal::enable_raw_mode()?;
    execute!(out, EnterAlternateScreen)?;
    Ok(())
}

fn exit_screen() -> io::Result<()> {
    let mut out = stdout();
    execute!(out, LeaveAlternateScreen)?;
    execute!(out, cursor::Show)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
