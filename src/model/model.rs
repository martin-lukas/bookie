use crate::{
    event::Message,
    image_util,
    model::{
        book::Book,
        book_info, book_table,
        persistance::{self, SavedState},
        status,
    },
};
use log::info;
use std::collections::HashSet;
use std::io;
use uuid::Uuid;

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
            book_info: book_info::State::new(image_util::create_picker()),
            status: status::State::new(),
            focus: Focus::Table,
            running_state: RunningState::Running,
        }
    }

    pub fn update(&mut self, msg: Message) -> Option<Message> {
        self.book_info.form.clear_error();
        match msg {
            Message::Quit => {
                self.persist();
                self.running_state = RunningState::Done;
            }
            Message::RefreshState => {
                self.persist();
                self.reload();
            }
            Message::NextBook => {
                self.select_next_book();
                self.persist();
            }
            Message::PreviousBook => {
                self.select_previous_book();
                self.persist();
            }
            Message::ConfirmDeleteBook => self.enter_confirm_mode(),
            Message::CancelConfirm => self.enter_view_mode(),
            Message::DeleteBook => {
                if let Some(book_index) = self.book_table.selected() {
                    self.delete_book(book_index);
                    self.enter_view_mode();
                    self.persist();
                }
            }
            Message::AddBook => self.enter_add_mode(),
            Message::EditBook => self.enter_edit_mode(),
            Message::CancelForm => self.enter_view_mode(),
            Message::InsertChar(c) => self.book_info.form.insert_char(c),
            Message::DeleteChar => self.book_info.form.delete_char(),
            Message::Increase => match self.book_info.form.cursor {
                4 => self.book_info.form.increase_reading_status(),
                5 => self.book_info.form.increase_rating(),
                _ => {}
            },
            Message::Decrease => match self.book_info.form.cursor {
                4 => self.book_info.form.decrease_reading_status(),
                5 => self.book_info.form.decrease_rating(),
                _ => {}
            },
            Message::NextFormField => self.book_info.form.next_field(),
            Message::PreviousFormField => self.book_info.form.previous_field(),
            Message::SubmitForm => match Book::from(&self.book_info.form) {
                Ok(mut book) => {
                    match self.book_info.mode {
                        book_info::Mode::Add => self.add_book(book),
                        book_info::Mode::Edit => self.update_book(&mut book),
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

    pub fn load() -> Self {
        let mut model = Self::from(persistance::load().expect("Failed to load state."));
        model.book_info.image_picker = image_util::create_picker();
        if let Some(new_book_index) = model.book_table.table_state.selected() {
            model.load_book_cover(new_book_index);
        }
        model
    }

    pub fn persist(&self) {
        persistance::save_state(&self).expect("Failed to save state.");
    }

    pub fn reload(&mut self) {
        let new = Self::load();
        self.books = new.books;
        self.book_table = new.book_table;
        self.book_info = new.book_info;
    }

    pub fn get_selected_book(&self) -> Option<&Book> {
        self.books.get(self.book_table.selected()?)
    }

    pub fn books_read(&self) -> usize {
        self.books.iter().filter(|b| b.is_read()).count()
    }

    pub fn unique_authors_read(&self) -> usize {
        self.books
            .iter()
            .filter(|b| b.is_read())
            .flat_map(|b| b.authors.clone())
            .collect::<HashSet<String>>()
            .len()
    }

    pub fn pages_read(&self) -> usize {
        self.books
            .iter()
            .filter(|b| b.is_read())
            .map(|b| b.pages as usize)
            .sum()
    }

    fn enter_add_mode(&mut self) {
        self.focus = Focus::Info;
        self.book_info.mode = book_info::Mode::Add;
        self.book_info.form = book_info::Form::default();
    }

    fn enter_edit_mode(&mut self) {
        if let Some(book) = self.get_selected_book() {
            self.book_info.form = book_info::Form::from(book);
            self.focus = Focus::Info;
            self.book_info.mode = book_info::Mode::Edit;
        }
    }

    fn enter_view_mode(&mut self) {
        self.focus = Focus::Table;
        self.status.mode = status::Mode::Ok;
        self.book_info.mode = book_info::Mode::View;
    }

    fn enter_confirm_mode(&mut self) {
        self.focus = Focus::Status;
        self.status.mode = status::Mode::ConfirmDeleteBook;
    }

    fn select_next_book(&mut self) {
        if let Some(current_book_index) = self.book_table.table_state.selected() {
            if current_book_index < self.books.len() - 1 {
                self.book_table.table_state.select_next();
                self.book_table.sync_scrollbar_position();
                if let Some(new_book_index) = self.book_table.table_state.selected() {
                    self.load_book_cover(new_book_index);
                }
            }
        }
    }

    fn select_previous_book(&mut self) {
        if let Some(book_index) = self.book_table.table_state.selected() {
            if book_index > 0 {
                self.book_table.table_state.select_previous();
                self.book_table.sync_scrollbar_position();
                if let Some(new_book_index) = self.book_table.table_state.selected() {
                    self.load_book_cover(new_book_index);
                }
            }
        }
    }

    fn select_book_by_index(&mut self, index: Option<usize>) {
        self.book_table.table_state.select(index);
        self.book_table.sync_scrollbar_position();
        if let Some(new_book_index) = self.book_table.table_state.selected() {
            self.load_book_cover(new_book_index);
        }
    }

    fn load_book_cover(&mut self, book_index: usize) {
        let book = &self.books[book_index];
        let Some(path) = &book.cover_path else {
            self.book_info.cover_image = None;
            return;
        };
        let dyn_img = image::ImageReader::open(path).and_then(|r| {
            r.decode()
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
        });
        match dyn_img {
            Ok(img) => {
                self.book_info.cover_image =
                    Some(self.book_info.image_picker.new_resize_protocol(img));
            }
            Err(_) => {
                self.book_info.cover_image = None;
            }
        }
    }

    fn add_book(&mut self, book: Book) {
        info!("Book added: {:?}", book);
        let id = book.id.clone();
        self.books.push(book);
        self.sort_books_by_title();
        self.select_book_by_id(id);
        self.update_scrollbar_length();
    }

    fn update_book(&mut self, updated_book: &mut Book) {
        if let Some(book_index) = self.book_table.selected() {
            updated_book.id = self.books[book_index].id;
            info!("Book updated: {:?}", updated_book);
            let book_id = updated_book.id.clone();
            self.books[book_index] = updated_book.to_owned();
            self.sort_books_by_title();
            self.select_book_by_id(book_id);
            self.update_scrollbar_length();
        } else {
            panic!("The book to be updated was not found")
        }
    }

    fn delete_book(&mut self, book_index: usize) {
        self.books.remove(book_index);
        if self.books.is_empty() {
            self.select_book_by_index(None)
        } else if book_index == self.books.len() {
            self.select_previous_book();
        }
        self.update_scrollbar_length();
    }

    fn update_scrollbar_length(&mut self) {
        self.book_table.update_scrollbar_length(self.books.len());
    }

    fn get_table_position_by_id(&self, book_id: Uuid) -> Option<usize> {
        self.books.iter().position(|b| b.id == book_id)
    }

    fn select_book_by_id(&mut self, id: Uuid) {
        match self.get_table_position_by_id(id) {
            Some(position) => self.select_book_by_index(Some(position)),
            None => {}
        }
    }

    fn sort_books_by_title(&mut self) {
        self.books
            .sort_by(|a, b| a.title_normalized().cmp(&b.title_normalized()));
    }
}

#[derive(Default, Eq, PartialEq)]
pub enum Focus {
    #[default]
    Table,
    Info,
    Status,
}

#[derive(Debug, Default, Eq, PartialEq)]
pub enum RunningState {
    #[default]
    Running,
    Done,
}
