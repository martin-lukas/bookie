pub mod reading_status;

use crate::model::{book::reading_status::ReadingStatus, book_info::form::BookForm};
use chrono::NaiveDate;
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
    pub finished_at: Vec<NaiveDate>,
    pub rating: u8,
    pub cover_path: Option<PathBuf>,
}

impl Book {
    pub fn from(form: &BookForm, existing_book: Option<&Book>) -> Result<Self, String> {
        let title = form.title.text.trim().to_string();
        if title.is_empty() {
            return Err("Title cannot be empty".to_string());
        }
        let year = form
            .year
            .text
            .trim()
            .parse::<u16>()
            .map_err(|_| "Year must be a valid number".to_string())?;
        let pages = form
            .pages
            .text
            .trim()
            .parse::<u16>()
            .map_err(|_| "Pages must be a valid number".to_string())?;
        let mut finished_at: Vec<NaiveDate> = vec![];
        let new_finished_at_str = form.finished_at.text.trim();
        if !new_finished_at_str.is_empty() {
            let new_finished_at: NaiveDate = new_finished_at_str
                .parse::<NaiveDate>()
                .map_err(|_| "Finished on must be a valid date in format YYYY-MM-DD")?;
            match existing_book {
                Some(existing_book) => match existing_book.finished_at.last() {
                    Some(most_recent_finished_at) => {
                        if new_finished_at > most_recent_finished_at.clone() {
                            finished_at = existing_book.finished_at.clone();
                            finished_at.push(new_finished_at);
                        } else {
                            return Err(
                                "New finished on date has to be more recent than the last."
                                    .to_string(),
                            );
                        }
                    }
                    None => {
                        finished_at.push(new_finished_at);
                    }
                },
                None => {
                    finished_at.push(new_finished_at);
                }
            }
        }

        let rating = form
            .rating
            .to_string()
            .trim()
            .parse::<u8>()
            .map_err(|_| "Rating must be a valid number".to_string())?;
        let authors: Vec<String> = form
            .authors
            .text
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        if authors.is_empty() {
            return Err("At least one author is required".to_string());
        }

        Ok(Self {
            id: form.id.unwrap_or(Uuid::new_v4()),
            title,
            authors,
            year,
            pages,
            reading_status: form.reading_status.clone(),
            finished_at,
            rating,
            cover_path: Some(PathBuf::from("./covers").join(format!("{}.jpg", form.title.text))),
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

    pub fn author_with_initials(author: &String) -> String {
        let parts: Vec<&str> = author.split_whitespace().collect();
        if parts.is_empty() {
            return String::new();
        }
        let (initials, surname) = parts.split_at(parts.len() - 1);
        let initials = initials
            .iter()
            .filter_map(|p| p.chars().next())
            .map(|c| format!("{}.", c))
            .collect::<Vec<_>>()
            .join(" ");
        if initials.is_empty() {
            surname[0].to_string()
        } else {
            format!("{} {}", initials, surname[0])
        }
    }
}
