use crate::{
    domain::{book::Book, book_form::BookForm, view::View},
    persistance::SavedState,
};
use log::info;

pub struct App {
    pub books: Vec<Book>,
    pub selected: usize,
    pub active_view: View,
    pub should_refresh: bool,
    pub add_book_form: Option<BookForm>,
    pub should_quit: bool,
}

impl App {
    pub fn new(saved_state: SavedState) -> App {
        App {
            books: saved_state.books,
            selected: saved_state.selected,
            active_view: saved_state.view,
            should_quit: false,
            should_refresh: false,
            add_book_form: None,
        }
    }

    pub fn move_selected(&mut self, delta: i64) {
        let mut new_selected = self.selected as i64 + delta;
        new_selected = new_selected.clamp(0, (self.books.len() - 1) as i64);
        info!(
            "Selected book index change: {} -> {}",
            self.selected, new_selected
        );
        self.selected = new_selected as usize;
    }

    pub fn change_view(&mut self, new_view: View) {
        info!("View change: {:?} -> {:?}", self.active_view, new_view);
        self.active_view = new_view;
        self.should_refresh = true;
    }

    pub fn add_book(&mut self, book: Book) {
        info!("New book added: {:?}", book);
        self.books.push(book);
        self.books.sort_by(|a, b| a.title.cmp(&b.title));
        self.add_book_form = None;
    }
}
