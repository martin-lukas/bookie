use crate::{
    domain::{book::Book, view::View},
    persistance::SavedState,
};

pub struct App {
    pub books: Vec<Book>,
    pub selected: usize,
    pub view: View,
    pub view_changed: bool,
    pub add_book_form: Option<AddBookForm>,
    pub should_quit: bool,
}

impl App {
    pub fn new(saved_state: SavedState) -> App {
        App {
            books: saved_state.books,
            selected: saved_state.selected,
            view: saved_state.view,
            should_quit: false,
            view_changed: false,
            add_book_form: None,
        }
    }
}

#[derive(Debug)]
pub struct AddBookForm {
    pub title: String,
    pub author: String,
    pub year: String,
    pub active: Field,
}

#[derive(Debug)]
pub enum Field {
    Title,
    Author,
    Year,
}

impl Field {
    pub fn index(self) -> usize {
        match self {
            Field::Title => 0,
            Field::Author => 1,
            Field::Year => 2,
        }
    }
}
