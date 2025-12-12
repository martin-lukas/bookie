use crossterm::terminal::ClearType;
use crossterm::{cursor, execute, terminal};
use std::io;
use std::io::stdout;

pub fn reset_screen() -> Result<(), io::Error> {
    execute!(
        stdout(),
        cursor::MoveTo(0, 0),
        terminal::Clear(ClearType::All)
    )
}
