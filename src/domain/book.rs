use crate::domain::book_form::BookForm;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Book {
    pub id: Uuid,
    pub title: String,
    pub author: String,
    pub year: u16,
    pub rating: u8,
    pub note: String,
}

impl Book {
    pub fn new(form: &BookForm) -> Self {
        Self {
            id: Uuid::new_v4(),
            title: form.title.to_string(),
            author: form.author.to_string(),
            year: form.year.parse::<u16>().unwrap(),
            rating: form.rating,
            note: form.note.to_string(),
        }
    }

    pub fn empty() -> Self {
        Self {
            id: Uuid::new_v4(),
            title: String::new(),
            author: String::new(),
            year: 0,
            rating: 1,
            note: String::new(),
        }
    }
}
