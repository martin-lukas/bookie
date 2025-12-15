use crate::{book::Book, view::View};
use crate::persistance::SavedState;

pub struct App {
    pub books: Vec<Book>,
    pub selected: usize,
    pub view: View,
    pub should_quit: bool,
    pub view_changed: bool,
}

impl App {
    pub fn new(saved_state: SavedState) -> App {
        App {
            books: saved_state.books,
            selected: saved_state.selected,
            view: saved_state.view,
            should_quit: false,
            view_changed: false,
        }
    }
}
