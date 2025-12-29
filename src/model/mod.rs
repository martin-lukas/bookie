pub mod book;
pub mod book_info;
pub mod book_table;
pub mod focus;
mod persistance;
pub mod running_state;
pub mod status;

use crate::{
    event::Message,
    image_util,
    model::{
        book::{reading_status::ReadingStatus, Book},
        book_info::{
            form::BookForm, form_field::FormField, BookInfoMode, BookInfoState, CoverStatus,
        },
        book_table::BookTableState,
        focus::Focus,
        persistance::SavedState,
        running_state::RunningState,
        status::StatusMode,
    },
};
use chrono::Datelike;
use log::info;
use ratatui_image::thread::{ResizeRequest, ResizeResponse, ThreadProtocol};
use std::{collections::HashSet, io, sync::mpsc};
use uuid::Uuid;

pub struct Model {
    pub books: Vec<Book>,
    pub book_table: BookTableState,
    pub book_info: BookInfoState,
    pub status: status::State,
    pub focus: Focus,
    pub running_state: RunningState,
}

impl Model {
    pub fn from(saved_state: SavedState) -> Self {
        let book_count = saved_state.books.len();
        Self {
            books: saved_state.books,
            book_table: BookTableState::new(book_count, saved_state.selected),
            book_info: BookInfoState::new(image_util::create_picker()),
            status: status::State::new(),
            focus: Focus::Table,
            running_state: RunningState::Running,
        }
    }

    pub fn update(&mut self, msg: Message) -> Option<Message> {
        self.clear_error();
        match msg {
            Message::Quit => {
                self.persist();
                self.running_state = RunningState::Done;
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
            Message::NewLineChar => match self.book_info.form.active {
                FormField::Note => self.book_info.form.insert_char('\n'),
                _ => return Some(Message::SubmitForm),
            },
            Message::DeleteChar => self.book_info.form.delete_char(),
            Message::FormLeft => match self.book_info.form.active {
                FormField::ReadingStatus => self.book_info.form.decrease_reading_status(),
                FormField::Rating => self.book_info.form.decrease_rating(),
                _ => self.book_info.form.move_cursor_left(),
            },
            Message::FormRight => match self.book_info.form.active {
                FormField::ReadingStatus => self.book_info.form.increase_reading_status(),
                FormField::Rating => self.book_info.form.increase_rating(),
                _ => self.book_info.form.move_cursor_right(),
            },
            Message::NextFormField => self.book_info.form.next_field(),
            Message::PreviousFormField => self.book_info.form.previous_field(),
            Message::SubmitForm => match Book::from(&self.book_info.form, self.get_selected_book())
            {
                Ok(mut book) => {
                    match self.book_info.mode {
                        BookInfoMode::Add => self.add_book(book),
                        BookInfoMode::Edit => self.update_book(&mut book),
                        BookInfoMode::View => {}
                    }
                    self.enter_view_mode();
                    self.persist();
                }
                Err(error) => {
                    self.book_info.form.error = Some(error.to_string());
                    self.status.mode = StatusMode::Error(error);
                }
            },
        }
        None
    }

    pub fn load() -> Self {
        let mut model = Self::from(persistance::load().expect("Failed to load state."));
        model.book_info.image_picker = image_util::create_picker();
        if let Some(new_book_index) = model.book_table.table_state.selected() {
            model.load_book_cover_async(new_book_index);
        }
        model.books.iter_mut().for_each(|book| {
            book.cover_path = Some(format!("./covers/{}.jpg", book.title).into());
        });
        model
    }

    pub fn persist(&self) {
        persistance::save_state(&self).expect("Failed to save state.");
    }

    pub fn get_selected_book(&self) -> Option<&Book> {
        self.books.get(self.book_table.selected()?)
    }

    pub fn unique_authors(&self) -> usize {
        self.books
            .iter()
            .flat_map(|b| b.authors.clone())
            .collect::<HashSet<String>>()
            .len()
    }

    pub fn books_read(&self) -> usize {
        self.books
            .iter()
            .filter(|b| !b.finished_at.is_empty())
            .count()
    }

    pub fn books_reading(&self) -> usize {
        self.books
            .iter()
            .filter(|b| b.reading_status == ReadingStatus::Reading)
            .count()
    }

    pub fn books_to_read(&self) -> usize {
        self.books
            .iter()
            .filter(|b| b.reading_status == ReadingStatus::ToRead)
            .count()
    }

    pub fn books_read_in_year(&self, year: u16) -> usize {
        self.books
            .iter()
            .map(|b| {
                b.finished_at
                    .iter()
                    .filter(|d| d.year() == (year as i32))
                    .count()
            })
            .sum()
    }

    pub fn pages_read_in_year(&self, year: u16) -> usize {
        self.books
            .iter()
            .map(|b| {
                (b.pages as usize)
                    * b.finished_at
                        .iter()
                        .filter(|d| d.year() == (year as i32))
                        .count()
            })
            .sum()
    }

    fn enter_add_mode(&mut self) {
        self.focus = Focus::Info;
        self.book_info.mode = BookInfoMode::Add;
        self.book_info.form = BookForm::default();
        self.book_info.cover = CoverStatus::None;
    }

    fn enter_edit_mode(&mut self) {
        if let Some(book) = self.get_selected_book() {
            self.book_info.form = BookForm::from(book);
            self.focus = Focus::Info;
            self.book_info.mode = BookInfoMode::Edit;
        }
    }

    fn enter_view_mode(&mut self) {
        self.focus = Focus::Table;
        self.status.mode = StatusMode::Ok;
        self.book_info.mode = BookInfoMode::View;
        if let Some(book_index) = self.book_table.table_state.selected() {
            self.load_book_cover_async(book_index);
        }
    }

    fn enter_confirm_mode(&mut self) {
        self.focus = Focus::Status;
        self.status.mode = StatusMode::ConfirmDeleteBook;
    }

    fn select_next_book(&mut self) {
        if let Some(current_book_index) = self.book_table.table_state.selected() {
            if current_book_index < self.books.len() - 1 {
                self.book_table.table_state.select_next();
                self.book_table.sync_scrollbar_position();
                if let Some(new_book_index) = self.book_table.table_state.selected() {
                    self.load_book_cover_async(new_book_index);
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
                    self.load_book_cover_async(new_book_index);
                }
            }
        }
    }

    fn select_book_by_index(&mut self, index: Option<usize>) {
        self.book_table.table_state.select(index);
        self.book_table.sync_scrollbar_position();
        if let Some(new_book_index) = self.book_table.table_state.selected() {
            self.load_book_cover_async(new_book_index);
        }
    }

    fn load_book_cover_async(&mut self, book_index: usize) {
        self.book_info.cover = CoverStatus::Loading;

        let (tx_resize_req, rx_resize_req) = mpsc::channel::<ResizeRequest>();
        let (tx_resize_res, rx_resize_res) = mpsc::channel::<ResizeResponse>();

        std::thread::spawn(move || {
            while let Ok(req) = rx_resize_req.recv() {
                match req.resize_encode() {
                    Ok(res) => {
                        tx_resize_res.send(res).ok();
                    }
                    Err(_) => panic!("Failed to resize encode image"),
                }
            }
        });

        let Some(path) = &self.books[book_index].cover_path else {
            self.book_info.cover = CoverStatus::None;
            self.book_info.cover_rx = None;
            return;
        };
        let img = match image::ImageReader::open(path).and_then(|r| {
            r.decode()
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
        }) {
            Ok(img) => img,
            Err(_) => {
                self.book_info.cover = CoverStatus::None;
                self.book_info.cover_rx = None;
                return;
            }
        };
        let protocol = self.book_info.image_picker.new_resize_protocol(img);

        self.book_info.cover =
            CoverStatus::Ready(ThreadProtocol::new(tx_resize_req, Some(protocol)));

        self.book_info.cover_rx = Some(rx_resize_res);
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

    fn clear_error(&mut self) {
        self.book_info.form.error = None;
        self.status.mode = StatusMode::Ok;
    }
}
