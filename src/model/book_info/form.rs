use crate::model::{
    book::{reading_status::ReadingStatus, Book},
    book_info::{
        form_field::FormField, text_input::TextInput, DEFAULT_RATING, MAX_RATING, MIN_RATING,
    },
};
use uuid::Uuid;

#[derive(Debug)]
pub struct BookForm {
    pub id: Option<Uuid>,
    pub title: TextInput,
    pub authors: TextInput,
    pub year: TextInput,
    pub pages: TextInput,
    pub reading_status: ReadingStatus,
    pub finished_at: TextInput,
    pub rating: u8,
    pub note: TextInput,
    pub active: FormField,
    pub error: Option<String>,
}

impl BookForm {
    pub fn from(book: &Book) -> Self {
        Self {
            id: Some(book.id),
            title: TextInput::new(book.title.clone()),
            authors: TextInput::new(book.authors.join(", ")),
            year: TextInput::new(book.year.to_string()),
            pages: TextInput::new(book.pages.to_string()),
            reading_status: book.reading_status.clone(),
            finished_at: TextInput::new(
                book.finished_at
                    .last()
                    .map(|d| d.to_string())
                    .unwrap_or("".to_string()),
            ),
            rating: book.rating,
            note: TextInput::new(book.note.clone()).multiline(true),
            active: FormField::Title,
            error: None,
        }
    }

    pub fn default() -> Self {
        Self {
            id: None,
            title: TextInput::default(),
            authors: TextInput::default(),
            year: TextInput::default(),
            pages: TextInput::default(),
            reading_status: ReadingStatus::ToRead,
            finished_at: TextInput::default(),
            rating: DEFAULT_RATING,
            note: TextInput::default().multiline(true),
            active: FormField::Title,
            error: None,
        }
    }

    pub fn insert_char(&mut self, c: char) {
        match self.active {
            FormField::Title => self.title.insert_char(c),
            FormField::Authors => self.authors.insert_char(c),
            FormField::Year => self.year.insert_char(c),
            FormField::Pages => self.pages.insert_char(c),
            FormField::FinishedAt => self.finished_at.insert_char(c),
            FormField::Note => self.note.insert_char(c),
            _ => {}
        }
    }

    pub fn delete_char(&mut self) {
        match self.active {
            FormField::Title => self.title.delete_char(),
            FormField::Authors => self.authors.delete_char(),
            FormField::Year => self.year.delete_char(),
            FormField::Pages => self.pages.delete_char(),
            FormField::FinishedAt => self.finished_at.delete_char(),
            FormField::Note => self.note.delete_char(),
            _ => {}
        };
    }

    pub fn move_cursor_left(&mut self) {
        match self.active {
            FormField::Title => self.title.move_cursor_left(),
            FormField::Authors => self.authors.move_cursor_left(),
            FormField::Year => self.year.move_cursor_left(),
            FormField::Pages => self.pages.move_cursor_left(),
            FormField::FinishedAt => self.finished_at.move_cursor_left(),
            FormField::Note => self.note.move_cursor_left(),
            _ => {}
        }
    }

    pub fn move_cursor_right(&mut self) {
        match self.active {
            FormField::Title => self.title.move_cursor_right(),
            FormField::Authors => self.authors.move_cursor_right(),
            FormField::Year => self.year.move_cursor_right(),
            FormField::Pages => self.pages.move_cursor_right(),
            FormField::FinishedAt => self.finished_at.move_cursor_right(),
            FormField::Note => self.note.move_cursor_right(),
            _ => {}
        }
    }

    pub fn increase_reading_status(&mut self) {
        if self.active == FormField::ReadingStatus && self.reading_status != ReadingStatus::Read {
            self.reading_status = ReadingStatus::from(self.reading_status.index() + 1);
        }
    }

    pub fn decrease_reading_status(&mut self) {
        if self.active == FormField::ReadingStatus && self.reading_status != ReadingStatus::ToRead {
            self.reading_status = ReadingStatus::from(self.reading_status.index() - 1);
        }
    }

    pub fn increase_rating(&mut self) {
        if self.active == FormField::Rating && self.rating < MAX_RATING {
            self.rating = self.rating + 1;
        }
    }

    pub fn decrease_rating(&mut self) {
        if self.active == FormField::Rating && self.rating > MIN_RATING {
            self.rating = self.rating - 1;
        }
    }

    pub fn next_field(&mut self) {
        self.active = self.active.next();
    }

    pub fn previous_field(&mut self) {
        self.active = self.active.prev();
    }
}
