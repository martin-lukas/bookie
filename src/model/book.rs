use crate::model::book_info;
use log::error;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Book {
    pub id: Uuid,
    pub title: String,
    pub authors: Vec<String>,
    pub year: u16,
    pub pages: u16,
    pub reading_status: ReadingStatus,
    pub rating: u8,
    pub note: String,
    pub cover_path: Option<PathBuf>,
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
            reading_status: form.reading_status.clone(),
            rating,
            note,
            cover_path: None,
        })
    }

    pub fn title_normalized(&self) -> String {
        self.title
            .strip_prefix("The ")
            .or_else(|| self.title.strip_prefix("A "))
            .or_else(|| self.title.strip_prefix("An "))
            .unwrap_or(&self.title)
            .to_lowercase()
    }

    pub fn is_read(&self) -> bool {
        self.reading_status == ReadingStatus::Read
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub enum ReadingStatus {
    #[default]
    ToRead,
    Reading,
    Read,
}

impl ReadingStatus {
    pub fn index(&self) -> usize {
        match self {
            ReadingStatus::ToRead => 0,
            ReadingStatus::Reading => 1,
            ReadingStatus::Read => 2,
        }
    }

    pub fn from(index: usize) -> Self {
        match index {
            0 => ReadingStatus::ToRead,
            1 => ReadingStatus::Reading,
            2 => ReadingStatus::Read,
            _ => {
                error!("Invalid index for reading status: {index}");
                panic!("Invalid index for reading status");
            }
        }
    }
}
