use chrono::Datelike;

pub const MAX_RATING: u8 = 5;
pub const DEFAULT_RATING: u8 = 1;

#[derive(Debug)]
pub struct BookForm {
    pub title: String,
    pub author: String,
    pub year: String,
    pub rating: u8,
    pub active_field: Field,
    pub error: String,
}

impl BookForm {
    pub fn move_active(&mut self, delta: i8) {
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
            Field::Rating => (),
        }
    }
    pub fn remove_active_last_char(&mut self) {
        self.clear_error();
        match self.active_field {
            Field::Title => self.title.pop(),
            Field::Author => self.author.pop(),
            Field::Year => self.year.pop(),
            Field::Rating => None,
        };
    }

    pub fn change_rating(&mut self, delta: i8) {
        if self.active_field != Field::Rating {
            return;
        }
        self.clear_error();
        let new_rating = self.rating as i8 + delta;
        self.rating = new_rating.clamp(0, MAX_RATING as i8) as u8;
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
        if self.rating > MAX_RATING {
            return Some(format!("Rating should be at most {}", MAX_RATING));
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
