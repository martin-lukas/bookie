use crate::{
    event::Message,
    model::{
        book::Book,
        book_info, book_table,
        persistance::{self, SavedState},
        status,
    },
};
use log::info;
use uuid::Uuid;

#[derive(Clone, Default)]
pub struct Model {
    pub books: Vec<Book>,
    pub book_table: book_table::State,
    pub book_info: book_info::State,
    pub status: status::State,
    pub focus: Focus,
    pub running_state: RunningState,
}

impl Model {
    pub fn from(saved_state: SavedState) -> Self {
        let book_count = saved_state.books.len();
        Self {
            books: saved_state.books,
            book_table: book_table::State::new(book_count, saved_state.selected),
            book_info: book_info::State::new(),
            status: status::State::new(),
            focus: Focus::Table,
            running_state: RunningState::Running,
        }
    }

    pub fn load() -> Self {
        Self::from(persistance::load().expect("Failed to load state."))
    }

    pub fn persist(&self) {
        persistance::save_state(&self).expect("Failed to save state.");
    }

    pub fn reload(&mut self) {
        self.clone_from(&Self::from(
            persistance::load().expect("Failed to reload state."),
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
                if let Some(book_index) = self.book_table.selected() {
                    self.books.remove(book_index);
                    if self.books.is_empty() {
                        self.book_table.select(None)
                    } else if book_index == self.books.len() {
                        self.book_table.select_previous();
                    }
                    self.enter_view_mode();
                    self.persist();
                }
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
        if let Some(book) = self.get_selected_book() {
            self.book_info.form = book_info::Form::from(book);
            self.focus = Focus::Info;
            self.book_info.mode = book_info::Mode::Edit;
        }
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

    pub fn get_selected_book(&self) -> Option<&Book> {
        self.books.get(self.book_table.selected()?)
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
        if let Some(book_index) = self.book_table.selected() {
            updated_book.id = self.books[book_index].id;
            info!("Book updated: {:?}", updated_book);
            let book_id = updated_book.id.clone();
            self.books[book_index] = updated_book.to_owned();
            self.sort_books_by_title();
            book_id
        } else {
            panic!("The book to be updated was not found")
        }
    }

    fn get_table_position_by_id(&self, book_id: Uuid) -> Option<usize> {
        self.books.iter().position(|b| b.id == book_id)
    }

    fn sort_books_by_title(&mut self) {
        self.books.sort_by(|a, b| a.title.cmp(&b.title));
    }
}

#[derive(Clone, Default, Eq, PartialEq)]
pub enum Focus {
    #[default]
    Table,
    Info,
    Status,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub enum RunningState {
    #[default]
    Running,
    Done,
}
