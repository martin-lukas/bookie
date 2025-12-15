use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum View {
    BookList,
    BookDetail,
    AddBook,
}
