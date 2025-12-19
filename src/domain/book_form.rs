use crate::domain::book::Book;
use chrono::Datelike;
use uuid::Uuid;

pub const MAX_RATING: u8 = 5;
pub const MIN_RATING: u8 = 1;
pub const DEFAULT_RATING: u8 = 3;

#[derive(Clone, Debug)]
pub struct BookForm {
    pub id: Option<Uuid>,
    pub title: String,
    pub author: String,
    pub year: String,
    pub pages: String,
    pub rating: u8,
    pub note: String,
    pub active_field: Field,
    pub error: String,
}

impl BookForm {
    pub fn new(book: &Book) -> Self {
        Self {
            id: Some(book.id),
            title: book.title.to_string(),
            author: book.author.to_string(),
            year: book.year.to_string(),
            pages: book.pages.to_string(),
            rating: book.rating,
            note: book.note.to_string(),
            active_field: Field::Rating,
            error: String::new(),
        }
    }

    pub fn empty() -> Self {
        Self {
            id: None,
            title: String::new(),
            author: String::new(),
            year: String::new(),
            pages: String::new(),
            rating: DEFAULT_RATING,
            note: String::new(),
            active_field: Field::Title,
            error: String::new(),
        }
    }

    pub fn move_active_field(&mut self, delta: i8) {
        self.clear_error();
        let mut new_active = self.active_field as i8 + delta;
        new_active = new_active.clamp(0, (Field::COUNT - 1) as i8);
        self.active_field = Field::get_by_index(new_active as usize);
    }

    pub fn add_active_char(&mut self, c: char) {
        self.clear_error();
        match self.active_field {
            Field::Title => self.title.push(c),
            Field::Author => self.author.push(c),
            Field::Year => self.year.push(c),
            Field::Pages => self.pages.push(c),
            Field::Rating => (),
            Field::Note => self.note.push(c),
        }
    }

    pub fn remove_active_last_char(&mut self) {
        self.clear_error();
        match self.active_field {
            Field::Title => self.title.pop(),
            Field::Author => self.author.pop(),
            Field::Year => self.year.pop(),
            Field::Pages => self.pages.pop(),
            Field::Rating => None,
            Field::Note => self.note.pop(),
        };
    }

    pub fn change_rating(&mut self, delta: i8) {
        if self.active_field != Field::Rating {
            return;
        }
        self.clear_error();
        let new_rating = self.rating as i8 + delta;
        self.rating = new_rating.clamp(MIN_RATING as i8, MAX_RATING as i8) as u8;
    }

    pub fn is_valid(&self) -> Option<String> {
        if self.title.is_empty() {
            return Some("Title shouldn't be empty".to_string());
        }
        if self.author.is_empty() {
            return Some("Author shouldn't be empty".to_string());
        }
        let current_year = chrono::Utc::now().year() as u16;
        if self.year.is_empty()
            || self
                .year
                .parse::<u16>()
                .map(|y| !(0..=current_year).contains(&y))
                .unwrap_or(true)
        {
            return Some(format!(
                "Year should be a number between 0 and {}",
                current_year
            ));
        }
        if self.rating < MIN_RATING || self.rating > MAX_RATING {
            return Some(format!(
                "Rating should be between {} and {}",
                MIN_RATING, MAX_RATING
            ));
        }
        None
    }

    pub fn clear_error(&mut self) {
        self.error = String::new();
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Field {
    Title,
    Author,
    Year,
    Pages,
    Rating,
    Note,
}

impl Field {
    pub const COUNT: usize = 6;

    pub fn index(&self) -> usize {
        match self {
            Field::Title => 0,
            Field::Author => 1,
            Field::Year => 2,
            Field::Pages => 3,
            Field::Rating => 4,
            Field::Note => 5,
        }
    }

    pub fn get_by_index(index: usize) -> Self {
        match index {
            0 => Field::Title,
            1 => Field::Author,
            2 => Field::Year,
            3 => Field::Pages,
            4 => Field::Rating,
            5 => Field::Note,
            _ => panic!("Invalid Field index: {}", index),
        }
    }
}

pub enum FormAction {
    None,
    AddChar(char),
    RemoveChar,
    VerticalMove(i8),
    ChangeRating(i8),
    Error(String),
    BackToList,
    Submit,
}
