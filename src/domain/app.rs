use crate::{
    domain::{book::Book, book_form::BookForm, layout::Layout, layout::Pane, view::View},
    persistance::SavedState,
};
use log::info;
use std::collections::HashMap;
use uuid::Uuid;

pub struct App {
    pub books: Vec<Book>,
    pub layout: Layout,
    pub view_map: HashMap<Pane, View>,
    pub selected: usize,
    pub book_form: BookForm,
    pub should_refresh: bool,
    pub should_quit: bool,
}

impl App {
    pub fn new(saved_state: SavedState, layout: Layout) -> App {
        let mut view_map = HashMap::new();
        view_map.insert(Pane::Top, View::BookList);
        view_map.insert(Pane::Bottom, View::BookDetail);
        view_map.insert(Pane::Right, View::BookStats);
        let book_form = BookForm::new(&saved_state.books[saved_state.selected]);
        App {
            books: saved_state.books,
            layout,
            view_map,
            selected: saved_state.selected,
            book_form,
            should_quit: false,
            should_refresh: false,
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

    pub fn change_view(&mut self, pane: Pane, view: View) {
        self.view_map.insert(pane, view);
    }

    pub fn change_focus(&mut self, focus_to: Pane) {
        info!(
            "Focus switch to pane: {:?} -> {:?}",
            self.layout.focused, focus_to
        );
        self.layout.focused = focus_to;
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
