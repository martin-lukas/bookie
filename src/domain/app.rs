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
    pub book_form: Option<BookForm>,
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
            book_form: None,
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

    pub fn sort_books_by_title(&mut self) {
        self.books.sort_by(|a, b| a.title.cmp(&b.title));
    }

    pub fn add_book(&mut self, book: Book) {
        info!("Book added: {:?}", book);
        self.books.push(book);
        self.sort_books_by_title();
        self.book_form = None;
    }

    pub fn update_selected_book(&mut self, form: &BookForm) {
        match self.books.get(self.selected) {
            Some(original_book) => {
                let mut updated_book = Book::new(form);
                updated_book.id = original_book.id;
                info!("Book updated: {:?}", updated_book);
                self.books[self.selected] = updated_book;
                self.sort_books_by_title();
                self.book_form = None;
            }
            None => (),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_state() -> SavedState {
        SavedState::empty()
    }

    #[test]
    fn move_selected_clamps_to_zero() {
        let mut app = App::new(test_state());
        let mut book_a = Book::empty();
        book_a.title = "Book A".to_string();
        let mut book_b = Book::empty();
        book_b.title = "Book B".to_string();
        app.books = vec![book_a, book_b];
        app.selected = 0;

        app.move_selected(-1);

        assert_eq!(app.selected, 0);
    }
}
