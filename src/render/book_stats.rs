use std::io;
use std::io::stdout;
use crossterm::cursor::MoveTo;
use crossterm::execute;
use crossterm::style::Print;
use crate::domain::app::App;

pub fn render(app: &App) -> io::Result<()> {
    let mut out = stdout();
    let rect = &app.layout.right; // TODO: Hardcoded?
    execute!(out, MoveTo(rect.x, rect.y), Print("hello"))?;
    Ok(())
}
