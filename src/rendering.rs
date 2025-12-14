use crate::book::Book;
use crate::view::View;
use crossterm::style::{PrintStyledContent, Stylize};
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    execute,
    style::Print,
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{self, stdout};

pub struct Renderer;

impl Renderer {
    pub fn init_screen() -> io::Result<()> {
        let mut out = stdout();
        terminal::enable_raw_mode()?;
        execute!(out, EnterAlternateScreen)?;
        Ok(())
    }

    pub fn exit_screen() -> io::Result<()> {
        let mut out = stdout();
        execute!(out, LeaveAlternateScreen)?;
        execute!(out, Show)?;
        terminal::disable_raw_mode()?;
        Ok(())
    }

    pub fn render(
        view: &View,
        books: &Vec<Book>,
        selected: usize,
        view_changed: bool,
    ) -> io::Result<()> {
        Renderer::reset_screen(view_changed)?;
        match view {
            View::List => Renderer::render_list(&books, &selected)?,
            View::Detail => match books.get(selected) {
                Some(book) => Renderer::render_detail(&book)?,
                None => panic!("Non-existent book selected for rendering!"),
            },
        }
        Ok(())
    }

    pub fn render_list(books: &Vec<Book>, selected: &usize) -> io::Result<()> {
        let mut out = stdout();
        for (i, book) in books.iter().enumerate() {
            execute!(out, MoveTo(0, i as u16))?;
            if i.eq(selected) {
                execute!(
                    out,
                    PrintStyledContent(
                        format!("> {} ({}) [{}]\n", book.title, book.author, book.year)
                            .bold()
                            .yellow()
                    ),
                )?;
            } else {
                execute!(
                    out,
                    Print(format!(
                        "  {} ({}) [{}]\n",
                        book.title, book.author, book.year
                    ))
                )?;
            }
        }
        Ok(())
    }

    pub fn render_detail(book: &Book) -> io::Result<()> {
        let mut out = stdout();
        execute!(
            out,
            MoveTo(0, 0),
            Print(format!("\tTitle: {}\n", book.title)),
        )?;
        execute!(
            out,
            MoveTo(0, 1),
            Print(format!("\tAuthor: {}\n", book.author)),
        )?;
        execute!(out, MoveTo(0, 2), Print(format!("\tYear: {}\n", book.year)),)?;

        Ok(())
    }

    pub fn reset_screen(should_clear: bool) -> io::Result<()> {
        let mut out = stdout();
        if should_clear {
            execute!(out, Clear(ClearType::All))?;
        }
        execute!(out, Hide, MoveTo(0, 0))
    }
}
