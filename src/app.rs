use crate::book::Book;

pub enum View {
    List,
}

pub struct App {
    pub books: Vec<Book>,
    pub selected: usize,
    pub view: View,
}

impl App {
    pub fn new(books: Vec<Book>) -> Self {
        Self {
            books,
            selected: 0,
            view: View::List,
        }
    }
}
