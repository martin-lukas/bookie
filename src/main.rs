mod app;
mod book;
mod logging;
mod persistance;
mod renderer;

use crate::app::App;
use crate::renderer::Renderer;
use crossterm::event::{Event, KeyCode};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{cursor, event, execute, terminal};
use std::io;
use std::io::stdout;

fn main() -> io::Result<()> {
    logging::setup_logger().expect("Failed to setup logger");

    init_screen()?;

    let books = persistance::load_books("books.json")?;

    let mut app = App::new(books);

    loop {
        Renderer::draw(&app)?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Up => {
                    if app.selected > 0 {
                        app.selected -= 1;
                    }
                }
                KeyCode::Down => {
                    if app.selected + 1 < app.books.len() {
                        app.selected += 1;
                    }
                }
                KeyCode::Char('q') => break,
                _ => {}
            }
        }
    }

    exit_screen()
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
