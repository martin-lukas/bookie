use crate::domain::view::View;

pub struct Layout {
    pub list: Pane,
    pub detail: Pane,
}

impl Layout {
    pub fn empty() -> Self {
        Self {
            list: Pane::new(View::BookList, Rect::empty(), false),
            detail: Pane::new(View::BookDetail, Rect::empty(), false),
        }
    }
}

pub struct Pane {
    pub view: View,
    pub area: Rect,
    pub is_focused: bool,
}

impl Pane {
    pub fn new(view: View, area: Rect, is_focused: bool) -> Self {
        Self {
            view,
            area,
            is_focused,
        }
    }
}

pub struct Rect {
    pub x: u16,
    pub y: u16,
    pub x_max: u16,
    pub y_max: u16,
}

impl Rect {
    pub fn new(x: u16, y: u16, x_max: u16, y_max: u16) -> Self {
        Self { x, y, x_max, y_max }
    }

    pub fn empty() -> Self {
        Self::new(0, 0, 0, 0)
    }
}
