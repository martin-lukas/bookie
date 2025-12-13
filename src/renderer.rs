use crate::app::{App, View};
use crossterm::{cursor, execute, style::Print};
use cursor::{Hide, MoveTo};
use std::io::{self, stdout};

pub struct Renderer;

impl Renderer {
    pub fn draw(app: &App) -> io::Result<()> {
        let mut out = stdout();

        execute!(out, Hide, MoveTo(0, 0))?;

        match app.view {
            View::List => {
                Renderer::render_list(app)?;
            }
        }

        Ok(())
    }

    fn render_list(app: &App) -> io::Result<()> {
        let mut out = stdout();
        for (i, book) in app.books.iter().enumerate() {
            if i == app.selected {
                execute!(
                    out,
                    MoveTo(0, i as u16),
                    Print(format!(
                        "> {} ({}) [{}]\n",
                        book.title, book.author, book.year
                    )),
                )?;
            } else {
                execute!(
                    out,
                    MoveTo(0, i as u16),
                    Print(format!(
                        "  {} ({}) [{}]\n",
                        book.title, book.author, book.year
                    ))
                )?;
            }
        }
        Ok(())
    }
}
