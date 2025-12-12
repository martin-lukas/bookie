mod logging;
mod app;
mod util;

use crossterm::event::Event;
use crossterm::terminal::{ClearType, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{cursor, event, execute, terminal};
use std::io;
use std::io::{stdout, Stdout};
use crate::app::App;

fn main() -> io::Result<()> {
    logging::setup_logger().expect("Failed to setup logger");

    let mut stdout = stdout();
    init_screen(&mut stdout)?;

    let mut app = App::new();
    app.render()?;

    println!("\nPress any key to exit...");

    loop {
        if event::poll(std::time::Duration::from_millis(200))? {
            if let Event::Key(_) = event::read()? {
                break;
            }
        }
    }

    exit_screen(&mut stdout)
}

fn init_screen(stdout: &mut Stdout) -> io::Result<()> {
    terminal::enable_raw_mode()?;
    execute!(stdout, cursor::Hide)?;
    execute!(stdout, EnterAlternateScreen)?;
    execute!(
        stdout,
        terminal::Clear(ClearType::All),
        cursor::MoveTo(0, 0)
    )?;
    Ok(())
}

fn exit_screen(stdout: &mut Stdout) -> io::Result<()> {
    execute!(stdout, LeaveAlternateScreen)?;
    execute!(stdout, cursor::Show)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
