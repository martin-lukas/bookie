use crate::{app::App, view::View};
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    execute,
    style::{Print, PrintStyledContent, Stylize},
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

    pub fn render(app: &App) -> io::Result<()> {
        Renderer::reset_screen(app.view_changed)?;
        match app.view {
            View::List => Renderer::render_list(&app)?,
            View::Detail => Renderer::render_detail(&app)?,
        }
        Ok(())
    }

    pub fn render_list(app: &App) -> io::Result<()> {
        let mut out = stdout();
        for (i, book) in app.books.iter().enumerate() {
            execute!(out, MoveTo(0, i as u16))?;
            if i == app.selected {
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

    pub fn render_detail(app: &App) -> io::Result<()> {
        if let Some(book) = app.books.get(app.selected) {
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
        } else {
            panic!("Non-existent book selected for rendering!")
        }
    }

    pub fn reset_screen(should_clear: bool) -> io::Result<()> {
        let mut out = stdout();
        if should_clear {
            execute!(out, Clear(ClearType::All))?;
        }
        execute!(out, Hide, MoveTo(0, 0))
    }
}
