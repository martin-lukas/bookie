use crossterm::{
    cursor::MoveTo,
    execute,
    terminal::{Clear, ClearType},
};
use std::io::{self, Write};

pub struct Layout {
    pub top: Rect,
    pub bottom: Rect,
    pub right: Rect,
    pub focused: Pane,
}

impl Layout {
    pub fn empty() -> Self {
        Self {
            top: Rect::empty(),
            bottom: Rect::empty(),
            right: Rect::empty(),
            focused: Pane::Top,
        }
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

    pub fn clear(&self, out: &mut impl Write) -> io::Result<()> {
        for i in 0..self.height {
            execute!(
                out,
                MoveTo(self.x, self.y + i),
                Clear(ClearType::CurrentLine),
            )?;
        }
        Ok(())
    }
}
