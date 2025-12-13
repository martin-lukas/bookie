use crate::book::Book;
use serde_json;
use std::fs::{self, File};
use std::io::Write;

pub fn load_books(path: &str) -> std::io::Result<Vec<Book>> {
    let data = fs::read_to_string(path)?;

    let books: Vec<Book> = serde_json::from_str(&data).expect("Failed to parse JSON");
    Ok(books)
}

pub fn save_books(books: &[Book], path: &str) -> std::io::Result<()> {
    let json = serde_json::to_string_pretty(books).expect("Failed to serialize books");
    let mut file = File::create(path)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}
