use crate::View;
use crossterm::{cursor, execute, style::Print};
use cursor::{Hide, MoveTo};
use std::io::{self, stdout};

pub struct Renderer;

impl Renderer {
    pub fn draw(view: &View) -> io::Result<()> {
        let mut out = stdout();

        execute!(out, Hide, MoveTo(0, 0))?;

        Renderer::render_list(view)?;

        Ok(())
    }

    fn render_list(view: &View) -> io::Result<()> {
        let mut out = stdout();
        for (i, book) in view.books.iter().enumerate() {
            if i == view.selected {
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
