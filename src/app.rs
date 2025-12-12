use crate::util::reset_screen;
use crossterm::{cursor, execute};
use std::io;
use std::io::stdout;

pub struct App {
    pub books: Vec<String>,
    pub cursor_row: u16,
}

impl App {
    pub fn new() -> Self {
        Self {
            books: vec![
                "The Hobbit".to_string(),
                "Dune".to_string(),
                "1984".to_string(),
                "The Catcher in the Rye".to_string(),
                "The Rust Programming Language".to_string(),
            ],
            cursor_row: 0,
        }
    }

    pub fn render(&mut self) -> io::Result<()> {
        let mut out = stdout();
        reset_screen()?;

        println!("📚 Books:\n");
        self.cursor_row += 2;
        execute!(out, cursor::MoveTo(0, self.cursor_row))?;

        for (i, b) in self.books.iter().enumerate() {
            println!("  {}. {}", i + 1, b);
            self.cursor_row += 1;
            execute!(out, cursor::MoveTo(0, self.cursor_row))?;
        }

        Ok(())
    }
}
