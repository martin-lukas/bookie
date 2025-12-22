use crate::model::persistance::{self, SavedState};
use crate::model::status;
use crate::{
    event::Message,
    model::{book::Book, book_info, book_table},
};
use log::info;
use uuid::Uuid;

#[derive(Clone)]
pub struct Model {
    pub books: Vec<Book>,
    pub book_table: book_table::State,
    pub book_info: book_info::State,
    pub status: status::State,
    pub focus: Focus,
    pub running_state: RunningState,
}

impl Model {
    pub fn load() -> Self {
        Self::from(persistance::load_state().expect("Failed to load state."))
    }

    pub fn persist(&self) {
        persistance::save_state(&self).expect("Failed to save state.");
    }

    pub fn reload(&mut self) {
        self.clone_from(&Self::from(
            persistance::load_state().expect("Failed to reload state."),
        ));
    }

    pub fn update(&mut self, msg: Message) -> Option<Message> {
        self.book_info.form.clear_error();
        match msg {
            Message::Quit => {
                self.running_state = RunningState::Done;
            }
            Message::RefreshState => self.reload(),
            Message::NextBook => self.book_table.select_next(),
            Message::PreviousBook => self.book_table.select_previous(),
            Message::ConfirmDeleteBook => self.enter_confirm_mode(),
            Message::CancelConfirm => self.enter_view_mode(),
            Message::DeleteBook => {
                self.books.remove(self.book_table.selected_unsafe());
                if self.book_table.selected_unsafe() == self.books.len() {
                    self.book_table.select_previous();
                    // TODO: what if only 1 book? unselect()?
                    // model.table_state.select(None);
                }
                self.enter_view_mode();
                self.persist();
            }
            Message::AddBook => self.enter_add_mode(),
            Message::EditBook => self.enter_edit_mode(),
            Message::CancelForm => self.enter_view_mode(),
            Message::InsertChar(c) => self.book_info.form.insert_char(c),
            Message::DeleteChar => self.book_info.form.delete_char(),
            Message::IncreaseRating => self.book_info.form.increase_rating(),
            Message::DecreaseRating => self.book_info.form.decrease_rating(),
            Message::NextFormField => self.book_info.form.next_field(),
            Message::PreviousFormField => self.book_info.form.previous_field(),
            Message::SubmitForm => match Book::from(&self.book_info.form) {
                Ok(mut book) => {
                    match self.book_info.mode {
                        book_info::Mode::Add => {
                            let id = self.add_book(book);
                            self.select_book_by_id(id);
                        }
                        book_info::Mode::Edit => {
                            let id = self.update_book(&mut book);
                            self.select_book_by_id(id);
                        }
                        book_info::Mode::View => {}
                    }
                    self.enter_view_mode();
                    self.persist();
                }
                Err(error) => {
                    self.book_info.form.error = Some(error.to_string());
                    self.status.mode = status::Mode::Error(error);
                }
            },
        }
        None
    }

    pub fn enter_add_mode(&mut self) {
        self.focus = Focus::Info;
        self.book_info.mode = book_info::Mode::Add;
        self.book_info.form = book_info::Form::default();
    }

    pub fn enter_edit_mode(&mut self) {
        self.focus = Focus::Info;
        self.book_info.mode = book_info::Mode::Edit;
        self.book_info.form = book_info::Form::from(self.get_selected_book_unsafe());
    }

    pub fn enter_view_mode(&mut self) {
        self.focus = Focus::Table;
        self.status.mode = status::Mode::Ok;
        self.book_info.mode = book_info::Mode::View;
    }

    pub fn enter_confirm_mode(&mut self) {
        self.focus = Focus::Status;
        self.status.mode = status::Mode::ConfirmDeleteBook;
    }

    pub fn get_selected_book_unsafe(&self) -> &Book {
        &self.books[self.book_table.selected_unsafe()]
    }

    pub fn select_book_by_id(&mut self, id: Uuid) {
        match self.get_table_position_by_id(id) {
            Some(position) => self.book_table.select(Some(position)),
            None => {}
        }
    }

    pub fn add_book(&mut self, book: Book) -> Uuid {
        info!("Book added: {:?}", book);
        let id = book.id.clone();
        self.books.push(book);
        self.sort_books_by_title();
        id
    }

    pub fn update_book(&mut self, updated_book: &mut Book) -> Uuid {
        let selected = self.book_table.selected_unsafe();
        updated_book.id = self.books[selected].id;
        info!("Book updated: {:?}", updated_book);
        let book_id = updated_book.id.clone();
        self.books[selected] = updated_book.to_owned();
        self.sort_books_by_title();
        book_id
    }

    fn from(saved_state: SavedState) -> Self {
        Self {
            books: saved_state.books,
            book_table: book_table::State::new(saved_state.selected),
            book_info: book_info::State::new(),
            status: status::State::new(),
            focus: Focus::Table,
            running_state: RunningState::Running,
        }
    }

    fn get_table_position_by_id(&self, book_id: Uuid) -> Option<usize> {
        self.books.iter().position(|b| b.id == book_id)
    }

    fn sort_books_by_title(&mut self) {
        self.books.sort_by(|a, b| a.title.cmp(&b.title));
    }
}

#[derive(Clone, Eq, PartialEq)]
pub enum Focus {
    Table,
    Info,
    Status,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RunningState {
    Running,
    Done,
}
