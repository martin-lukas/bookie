use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub year: u16,
    pub rating: u8,
}

pub const STAR: &str = "⭑"; // ⭐/ ✰ / ★ / ⭑
