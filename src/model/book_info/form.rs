use crate::model::{
    book::{Book, ReadingStatus},
    book_info::{DEFAULT_RATING, MAX_RATING, MIN_RATING},
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
            rating: DEFAULT_RATING,
            note: TextInput::default().multiline(true),
            active: FormField::Title,
            error: None,
        }
    }

    pub fn insert_char(&mut self, c: char) {
        match self.active {
            FormField::Title => self.title.text.push(c),
            FormField::Authors => self.authors.text.push(c),
            FormField::Year => self.year.text.push(c),
            FormField::Pages => self.pages.text.push(c),
            FormField::Note => self.note.text.push(c),
            _ => {}
        }
    }

    pub fn delete_char(&mut self) {
        match self.active {
            FormField::Title => self.title.text.pop(),
            FormField::Authors => self.authors.text.pop(),
            FormField::Year => self.year.text.pop(),
            FormField::Pages => self.pages.text.pop(),
            FormField::Note => self.note.text.pop(),
            _ => None,
        };
        ()
    }

    pub fn move_cursor_left(&mut self) {
        match self.active {
            FormField::Title => {
                if self.title.cursor > 0 {
                    self.title.cursor -= 1;
                }
            }
            FormField::Authors => {
                if self.authors.cursor > 0 {
                    self.authors.cursor -= 1;
                }
            }
            FormField::Year => {
                if self.year.cursor > 0 {
                    self.year.cursor -= 1;
                }
            }
            FormField::Pages => {
                if self.pages.cursor > 0 {
                    self.pages.cursor -= 1;
                }
            }
            FormField::Note => {
                if self.note.cursor > 0 {
                    self.note.cursor -= 1;
                }
            }
            _ => {}
        }
    }

    pub fn move_cursor_right(&mut self) {
        match self.active {
            FormField::Title => {
                if self.title.cursor < self.title.text.len() {
                    self.title.cursor += 1;
                }
            }
            FormField::Authors => {
                if self.authors.cursor < self.authors.text.len() {
                    self.authors.cursor += 1;
                }
            }
            FormField::Year => {
                if self.year.cursor < self.year.text.len() {
                    self.year.cursor += 1;
                }
            }
            FormField::Pages => {
                if self.pages.cursor < self.pages.text.len() {
                    self.pages.cursor += 1;
                }
            }
            FormField::Note => {
                if self.note.cursor < self.note.text.len() {
                    self.note.cursor += 1;
                }
            }
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

    pub fn clear_error(&mut self) {
        self.error = None;
    }
}

#[derive(Debug, Default)]
pub struct TextInput {
    pub text: String,
    pub cursor: usize,
    pub multiline: bool,
}

impl TextInput {
    pub fn new(text: String) -> Self {
        Self {
            text,
            cursor: 0,
            multiline: false,
        }
    }

    pub fn multiline(mut self, multiline: bool) -> Self {
        self.multiline = multiline;
        self
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum FormField {
    Title,
    Authors,
    Year,
    Pages,
    ReadingStatus,
    Rating,
    Note,
}

impl FormField {
    pub const ORDER: [FormField; 7] = [
        FormField::Title,
        FormField::Authors,
        FormField::Year,
        FormField::Pages,
        FormField::ReadingStatus,
        FormField::Rating,
        FormField::Note,
    ];

    pub fn next(&self) -> Self {
        let pos = Self::ORDER.iter().position(|f| f == self).unwrap();
        Self::ORDER[(pos + 1) % Self::ORDER.len()]
    }

    pub fn prev(&self) -> Self {
        let pos = Self::ORDER.iter().position(|f| f == self).unwrap();
        let len = Self::ORDER.len();
        Self::ORDER[(pos + len - 1) % len]
    }
}
