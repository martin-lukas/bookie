use crate::{
    model::{book::Book, book_info, book_table},
    persistance::SavedState,
};
use log::info;
use uuid::Uuid;
use crate::event::Message;

pub struct Model {
    pub books: Vec<Book>,
    pub book_table: book_table::State,
    pub book_info: book_info::State,
    pub focus: Focus,
    pub running_state: RunningState,
}

impl Model {
    pub fn from(saved_state: SavedState) -> Model {
        Model {
            books: saved_state.books,
            book_table: book_table::State::new(saved_state.selected),
            book_info: book_info::State::new(),
            focus: Focus::Table,
            running_state: RunningState::Running,
        }
    }


    pub fn update(&mut self, msg: Message) -> Option<Message> {
        match msg {
            Message::Quit => {
                self.running_state = RunningState::Done;
            }
            Message::NextBook => self.book_table.select_next(),
            Message::PreviousBook => self.book_table.select_previous(),
            Message::DeleteBook => {
                // TODO: before activating - figure out barebones confirmation...
                self.books.remove(self.book_table.selected_unsafe());
                if self.book_table.selected_unsafe() == self.books.len() {
                    self.book_table.select_previous();
                    // TODO: what if only 1 book? unselect()?
                    // model.table_state.select(None);
                }
            }
            Message::AddBook => self.enter_add_mode(),
            Message::EditBook => self.enter_edit_mode(),
            Message::CancelForm => self.enter_view_mode(),
            Message::InsertChar(c) => self.book_info.form.insert_char(c),
            Message::DeleteChar => self.book_info.form.delete_char(),
            Message::NextFormField => self.book_info.form.next_field(),
            Message::PreviousFormField => self.book_info.form.previous_field(),
            Message::SubmitForm => {
                match Book::from(&self.book_info.form) {
                    Ok(book) => {
                        self.add_book(book);
                        self.enter_view_mode();
                    }
                    Err(error) => {
                        self.book_info.form.error = Some(error);
                    }
                }
            }
        }
        None
    }

    pub fn enter_add_mode(&mut self) {
        self.focus = Focus::Info;
        self.book_info.mode = book_info::Mode::Add;
        self.book_info.form = book_info::Form::default();
    }

    pub fn enter_edit_mode(&mut self) {
        self.focus = Focus::Info;
        self.book_info.mode = book_info::Mode::Edit;
        self.book_info.form =
            book_info::Form::from(&self.books[self.book_table.selected_unsafe()]);
    }

    pub fn enter_view_mode(&mut self) {
        self.focus = Focus::Table;
        self.book_info.mode = book_info::Mode::View;
    }

    pub fn add_book(&mut self, book: Book) -> Uuid {
        info!("Book added: {:?}", book);
        let id = book.id.clone();
        self.books.push(book);
        self.sort_books_by_title();
        id
    }

    // pub fn update_selected_book(&mut self, form: &book_info::Form) -> Uuid {
    //     let mut updated_book = Book::from(form);
    //     let selected = self.book_table.selected_unsafe();
    //     updated_book.id = self.books[selected].id;
    //     info!("Book updated: {:?}", updated_book);
    //     let book_id = updated_book.id.clone();
    //     self.books[selected] = updated_book;
    //     self.sort_books_by_title();
    //     book_id
    // }

    fn sort_books_by_title(&mut self) {
        self.books.sort_by(|a, b| a.title.cmp(&b.title));
    }
}

pub enum Focus {
    Table,
    Info,
}

#[derive(Debug, PartialEq, Eq)]
pub enum RunningState {
    Running,
    Done,
}
