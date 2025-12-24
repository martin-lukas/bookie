use crate::model::book::{Book, ReadingStatus};
use uuid::Uuid;

pub const MIN_RATING: u8 = 0; // haven't read yet
pub const MAX_RATING: u8 = 5;
pub const DEFAULT_RATING: u8 = 0;

#[derive(Clone, Default)]
pub struct State {
    pub mode: Mode,
    pub form: Form,
}

impl State {
    pub fn new() -> Self {
        Self {
            mode: Mode::View,
            form: Form::default(),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Mode {
    #[default]
    View,
    Add,
    Edit,
}

#[derive(Clone, Debug, Default)]
pub struct Form {
    pub id: Option<Uuid>,
    pub title: String,
    pub authors: String,
    pub year: String,
    pub pages: String,
    pub reading_status: ReadingStatus,
    pub rating: u8,
    pub note: String,
    pub cursor: usize,
    pub error: Option<String>,
}

const FORM_FIELD_COUNT: usize = 7;

impl Form {
    pub fn from(book: &Book) -> Self {
        Self {
            id: Some(book.id),
            title: book.title.to_string(),
            authors: book.authors.join(", ").to_string(),
            year: book.year.to_string(),
            pages: book.pages.to_string(),
            reading_status: book.reading_status.clone(),
            rating: book.rating,
            note: book.note.to_string(),
            cursor: 0,
            error: None,
        }
    }

    pub fn default() -> Self {
        Self {
            id: None,
            title: String::new(),
            authors: String::new(),
            year: String::new(),
            pages: String::new(),
            reading_status: ReadingStatus::ToRead,
            rating: DEFAULT_RATING,
            note: String::new(),
            cursor: 0,
            error: None,
        }
    }

    pub fn insert_char(&mut self, c: char) {
        match self.cursor {
            0 => self.title.push(c),
            1 => self.authors.push(c),
            2 => self.year.push(c),
            3 => self.pages.push(c),
            6 => self.note.push(c),
            _ => {}
        }
    }

    pub fn delete_char(&mut self) {
        match self.cursor {
            0 => self.title.pop(),
            1 => self.authors.pop(),
            2 => self.year.pop(),
            3 => self.pages.pop(),
            6 => self.note.pop(),
            _ => None,
        };
        ()
    }

    pub fn increase_reading_status(&mut self) {
        if self.cursor == 4 && self.reading_status != ReadingStatus::Read {
            self.reading_status = ReadingStatus::from(self.reading_status.index() + 1);
        }
    }

    pub fn decrease_reading_status(&mut self) {
        if self.cursor == 4 && self.reading_status != ReadingStatus::ToRead {
            self.reading_status = ReadingStatus::from(self.reading_status.index() - 1);
        }
    }

    pub fn increase_rating(&mut self) {
        if self.cursor == 5 && self.rating < MAX_RATING {
            self.rating = self.rating + 1;
        }
    }

    pub fn decrease_rating(&mut self) {
        if self.cursor == 5 && self.rating > MIN_RATING {
            self.rating = self.rating - 1;
        }
    }

    pub fn next_field(&mut self) {
        self.cursor = (self.cursor + 1) % FORM_FIELD_COUNT;
    }

    pub fn previous_field(&mut self) {
        self.cursor = self.cursor.checked_sub(1).unwrap_or(FORM_FIELD_COUNT - 1);
    }

    pub fn clear_error(&mut self) {
        self.error = None;
    }
}
