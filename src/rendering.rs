use crate::{app::App, view::View};
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    execute,
    style::{Print, PrintStyledContent, Stylize},
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{self, stdout};

const COL_ID: usize = 4;
const COL_TITLE: usize = 30;
const COL_AUTHOR: usize = 20;
const COL_YEAR: usize = 4;
const TABLE_WIDTH: usize = COL_ID + COL_TITLE + COL_AUTHOR + COL_YEAR;

fn pad(s: &str, width: usize) -> String {
    format!("{:<width$}", s, width = width)
}

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
        execute!(
            out,
            PrintStyledContent(
                format!(
                    "{}{}{}{}",
                    pad("#", COL_ID),
                    pad("Title", COL_TITLE),
                    pad("Author", COL_AUTHOR),
                    pad("Year", COL_YEAR),
                )
                .bold()
            ),
            MoveTo(0, 1),
            Print("-".repeat(TABLE_WIDTH)),
            MoveTo(0, 2),
        )?;

        for (i, book) in app.books.iter().enumerate() {
            if i == app.selected {
                execute!(
                    out,
                    PrintStyledContent(
                        format!(
                            "{}{}{}{}",
                            pad(&(i + 1).to_string(), COL_ID),
                            pad(&book.title, COL_TITLE),
                            pad(&book.author, COL_AUTHOR),
                            pad(&book.year.to_string(), COL_YEAR),
                        )
                        .bold()
                        .yellow()
                    ),
                )?;
            } else {
                execute!(
                    out,
                    Print(format!(
                        "{}{}{}{}",
                        pad(&(i + 1).to_string(), COL_ID),
                        pad(&book.title, COL_TITLE),
                        pad(&book.author, COL_AUTHOR),
                        pad(&book.year.to_string(), COL_YEAR),
                    ))
                )?;
            }
            execute!(out, MoveTo(0, (i + 3) as u16))?;
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
