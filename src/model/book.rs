use crate::model::book_info;
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
    pub fn from(form: &book_info::Form) -> Result<Self, String> {
        // Validate and parse year
        let year = form.year.trim().parse::<u16>()
            .map_err(|_| "Year must be a valid number".to_string())?;

        // Validate and parse pages
        let pages = form.pages.trim().parse::<u16>()
            .map_err(|_| "Pages must be a valid number".to_string())?;

        // Validate and parse rating (assume 1 to 5 stars)
        let rating = form.rating.to_string().trim().parse::<u8>()// todo: fix to_string baboness
            .map_err(|_| "Rating must be a valid number".to_string())?;

        if rating == 0 || rating > 5 {
            return Err("Rating must be between 1 and 5".to_string());
        }

        // Split authors by comma and trim whitespace
        let authors: Vec<String> = form.authors
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        if authors.is_empty() {
            return Err("At least one author is required".to_string());
        }

        // Title must not be empty
        if form.title.trim().is_empty() {
            return Err("Title cannot be empty".to_string());
        }

        Ok(Self {
            id: form.id.unwrap_or(Uuid::new_v4()),
            title: form.title.trim().to_string(),
            authors,
            year,
            pages,
            rating,
            note: form.note.to_string(),
        })

        // Self {
        //     id: Uuid::new_v4(),
        //     title: form.title.to_string(),
        //     authors: vec![],
        //     // authors: form.author.to_string(),
        //     year: form.year.parse::<u16>().unwrap(),
        //     pages: form.pages.parse::<u16>().unwrap(),
        //     rating: form.rating,
        //     note: form.note.to_string(),
        // }
    }
}
