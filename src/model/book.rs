use crate::model::book_info;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
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
    pub fn from(form: &book_info::Form) -> Result<Self, String> {
        let title = form.title.trim().to_string();
        if title.is_empty() {
            return Err("Title cannot be empty".to_string());
        }
        let year = form
            .year
            .trim()
            .parse::<u16>()
            .map_err(|_| "Year must be a valid number".to_string())?;
        let pages = form
            .pages
            .trim()
            .parse::<u16>()
            .map_err(|_| "Pages must be a valid number".to_string())?;
        let rating = form
            .rating
            .to_string()
            .trim()
            .parse::<u8>()
            .map_err(|_| "Rating must be a valid number".to_string())?;
        if rating == 0 || rating > 5 {
            return Err("Rating must be between 1 and 5".to_string());
        }
        let authors: Vec<String> = form
            .authors
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        if authors.is_empty() {
            return Err("At least one author is required".to_string());
        }
        let note = form.note.trim().to_string();

        Ok(Self {
            id: form.id.unwrap_or(Uuid::new_v4()),
            title,
            authors,
            year,
            pages,
            rating,
            note,
        })
    }
}
