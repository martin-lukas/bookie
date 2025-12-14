use crate::book::Book;

pub struct View<'a> {
    pub selected: usize,
    pub books: &'a Vec<Book>,
}

impl<'a> View<'a> {
    pub fn new(books: &'a Vec<Book>) -> Self {
        Self { selected: 0, books }
    }
}
