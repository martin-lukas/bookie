use crossterm::{
    cursor::MoveTo,
    execute,
    style::{Attribute, Print, ResetColor, SetAttribute},
};
use std::io::{self, stdout, Write};

pub struct Layout {
    pub cols: u16,
    pub rows: u16,
    pub top: Rect,
    pub bottom: Rect,
    pub right: Rect,
    pub focused: Pane,
}

impl Layout {
    pub fn empty() -> Self {
        Self {
            cols: 0,
            rows: 0,
            top: Rect::empty(),
            bottom: Rect::empty(),
            right: Rect::empty(),
            focused: Pane::Top,
        }
    }

    pub fn render_dividers(&self) -> io::Result<()> {
        let mut out = stdout();
        self.top.render_border()?;
        self.bottom.render_border()?;
        self.right.render_border()?;
        self.render_border_crossings()?;
        Ok(())
    }

    pub fn render_border_crossings(&self) -> io::Result<()> {
        execute!(
            stdout(),
            MoveTo(self.top.left_border(), self.top.bottom_border()),
            Print("├"),
            MoveTo(self.top.right_border(), self.top.top_border()),
            Print("┬"),
            MoveTo(self.top.right_border(), self.top.bottom_border()),
            Print("┤"),
            MoveTo(self.bottom.right_border(), self.bottom.bottom_border()),
            Print("┴")
        )?;
        Ok(())
    }

    pub fn clear_all(&self, out: &mut impl Write) -> io::Result<()> {
        self.top.clear(out)?;
        self.bottom.clear(out)?;
        self.right.clear(out)?;
        Ok(())
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum Pane {
    Top,
    Bottom,
    Right,
}

pub struct Rect {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

impl Rect {
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub fn empty() -> Self {
        Self::new(0, 0, 0, 0)
    }

    pub fn left_border(&self) -> u16 {
        self.x - 1
    }

    pub fn right_border(&self) -> u16 {
        self.x + self.width - 1
    }

    pub fn top_border(&self) -> u16 {
        self.y - 1
    }

    pub fn bottom_border(&self) -> u16 {
        self.y + self.height - 1
    }

    pub fn render_border(&self) -> io::Result<()> {
        let mut out = stdout();
        let x0 = self.left_border();
        let x1 = self.right_border();
        let y0 = self.top_border();
        let y1 = self.bottom_border();
        // Top and bottom border
        for x in x0 + 1..x1 {
            execute!(out, MoveTo(x, y0), Print("─"), MoveTo(x, y1), Print("─"))?;
        }
        // Left and right border
        for y in y0 + 1..y1 {
            execute!(out, MoveTo(x0, y), Print("│"), MoveTo(x1, y), Print("│"))?;
        }
        // Corners
        execute!(
            out,
            MoveTo(x0, y0),
            Print("┌"),
            MoveTo(x1, y0),
            Print("┐"),
            MoveTo(x0, y1),
            Print("└"),
            MoveTo(x1, y1),
            Print("┘"),
        )?;
        Ok(())
    }

    pub fn clear(&self, out: &mut impl Write) -> io::Result<()> {
        execute!(out, ResetColor, SetAttribute(Attribute::Reset))?;
        for row in 0..self.height {
            for col in 0..self.width {
                execute!(out, MoveTo(self.x + col, self.y + row), Print(" "))?;
            }
        }
        Ok(())
    }
}
