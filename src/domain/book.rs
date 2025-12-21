use crate::domain::book_form::BookForm;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Book {
    pub id: Uuid,
    pub title: String,
    pub authors: Vec<String>,
    pub year: u16,
    pub pages: u16,
    pub rating: u8,
    pub note: String,
}

impl Book {
    pub fn new(form: &BookForm) -> Self {
        Self {
            id: Uuid::new_v4(),
            title: form.title.to_string(),
            authors: vec![],
            // authors: form.author.to_string(),
            year: form.year.parse::<u16>().unwrap(),
            pages: form.pages.parse::<u16>().unwrap(),
            rating: form.rating,
            note: form.note.to_string(),
        }
    }
}
