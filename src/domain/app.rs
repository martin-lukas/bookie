use crate::{
    domain::{book::Book, book_form::BookForm, layout::Layout, view::View},
    persistance::SavedState,
};
use log::info;

pub struct App {
    pub books: Vec<Book>,
    pub selected: usize,
    pub layout: Layout,
    pub should_refresh: bool,
    pub book_form: Option<BookForm>,
    pub should_quit: bool,
}

impl App {
    pub fn new(saved_state: SavedState, layout: Layout) -> App {
        App {
            books: saved_state.books,
            selected: saved_state.selected,
            layout,
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

    pub fn change_detail_view(&mut self, new_view: View) {
        info!("Detail view change: {:?} -> {:?}", self.layout.detail.view, new_view);
        self.layout.detail.view = new_view;
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
    use rstest::rstest;

    fn test_state() -> SavedState {
        SavedState::empty()
    }

    #[rstest]
    #[case(0, 0, 0)]
    #[case(1, 1, 2)]
    #[case(2, 1, 2)]
    #[case(2, 0, 2)]
    #[case(1, -1, 0)]
    #[case(0, -1, 0)]
    fn move_selected_clamps_to_zero(
        #[case] start_position: usize,
        #[case] move_by: i64,
        #[case] end_position: usize,
    ) {
        let mut app = App::new(test_state(), Layout::empty());
        app.books = vec![Book::empty(), Book::empty(), Book::empty()];
        app.selected = start_position;

        app.move_selected(move_by);

        assert_eq!(app.selected, end_position);
    }
}
