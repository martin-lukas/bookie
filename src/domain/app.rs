use crate::{
    domain::{book::Book, view::View},
    persistance::SavedState,
};
use chrono::Datelike;

pub struct App {
    pub books: Vec<Book>,
    pub selected: usize,
    pub view: View,
    pub should_refresh: bool,
    pub add_book_form: Option<AddBookForm>,
    pub should_quit: bool,
}

impl App {
    pub fn new(saved_state: SavedState) -> App {
        App {
            books: saved_state.books,
            selected: saved_state.selected,
            view: saved_state.view,
            should_quit: false,
            should_refresh: false,
            add_book_form: None,
        }
    }

    pub fn add_book(&mut self, book: Book) {
        self.books.push(book);
        self.books.sort_by(|a, b| a.title.cmp(&b.title));
    }
}

#[derive(Debug)]
pub struct AddBookForm {
    pub title: String,
    pub author: String,
    pub year: String,
    pub rating: u8,
    pub active: Field,
    pub error: String,
}

impl AddBookForm {
    pub fn move_active(&mut self, delta: i8) {
        self.clear_error();
        let new_active = self.active as i8 + delta;
        self.active = Field::get_by_index(new_active as usize);
    }

    pub fn add_active_char(&mut self, c: char) {
        self.clear_error();
        match self.active {
            Field::Title => self.title.push(c),
            Field::Author => self.author.push(c),
            Field::Year => self.year.push(c),
            Field::Rating => (),
        }
    }
    pub fn remove_active_last_char(&mut self) {
        self.clear_error();
        match self.active {
            Field::Title => self.title.pop(),
            Field::Author => self.author.pop(),
            Field::Year => self.year.pop(),
            Field::Rating => None,
        };
    }

    pub fn change_rating(&mut self, delta: i8) {
        if self.active != Field::Rating {
            return;
        }
        self.clear_error();
        let new_rating = self.rating as i8 + delta;
        self.rating = new_rating.clamp(0, MAX_RATING as i8) as u8;
    }

    pub fn is_valid(&self) -> bool {
        !self.title.is_empty()
            && !self.author.is_empty()
            && !self.year.is_empty()
            && self
                .year
                .parse::<u16>()
                .map(|y| (0..=(chrono::Utc::now().year() as u16)).contains(&y))
                .unwrap_or(false)
            && self.rating <= MAX_RATING
    }

    pub fn clear_error(&mut self) {
        self.error = String::new();
    }
}

pub const MAX_RATING: u8 = 5;
pub const DEFAULT_RATING: u8 = 1;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Field {
    Title,
    Author,
    Year,
    Rating,
}

impl Field {
    pub const COUNT: usize = 4;

    pub fn index(&self) -> usize {
        match self {
            Field::Title => 0,
            Field::Author => 1,
            Field::Year => 2,
            Field::Rating => 3,
        }
    }

    pub fn get_by_index(index: usize) -> Self {
        match index {
            0 => Field::Title,
            1 => Field::Author,
            2 => Field::Year,
            3 => Field::Rating,
            _ => panic!("Invalid Field index: {}", index),
        }
    }
}
