use crate::domain::{book::Book, book_form::BookForm};
use log::info;
use ratatui::widgets::TableState;
use uuid::Uuid;
use crate::persistance::SavedState;

pub struct Model {
    pub books: Vec<Book>,
    pub selected: usize,
    pub book_form: BookForm,
    pub table_state: TableState,
    pub running_state: RunningState,
}

impl Model {
    pub fn from(saved_state: SavedState) -> Model {
        // let book_form = BookForm::new(&saved_state.books[saved_state.selected]);
        let mut table_state = TableState::default();
        table_state.select(Some(saved_state.selected));
        Model {
            books: saved_state.books,
            selected: saved_state.selected,
            book_form: BookForm::empty(),
            table_state,
            running_state: RunningState::Running,
        }
    }

    pub fn move_selected(&mut self, delta: i64) {
        if self.books.is_empty() {
            self.selected = 0;
            return;
        }

        let max = self.books.len() - 1;
        if delta.is_negative() {
            self.selected = self.selected.saturating_sub(delta.unsigned_abs() as usize);
        } else {
            self.selected = self.selected.saturating_add(delta as usize);
        }
        self.selected = self.selected.min(max);

        self.book_form = BookForm::new(&self.books[self.selected]);
    }

    pub fn sort_books_by_title(&mut self) {
        self.books.sort_by(|a, b| a.title.cmp(&b.title));
    }

    pub fn add_book(&mut self, book: Book) -> Uuid {
        info!("Book added: {:?}", book);
        let id = book.id.clone();
        self.books.push(book);
        self.sort_books_by_title();
        id
    }

    pub fn update_selected_book(&mut self, form: &BookForm) -> Uuid {
        let mut updated_book = Book::new(form);
        updated_book.id = self.books[self.selected].id;
        info!("Book updated: {:?}", updated_book);
        let book_id = updated_book.id.clone();
        self.books[self.selected] = updated_book;
        self.sort_books_by_title();
        book_id
    }
}


#[derive(Debug, PartialEq, Eq)]
pub enum RunningState {
    Running,
    Done,
}
