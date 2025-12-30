pub mod form;
pub mod form_field;
pub mod text_input;

use crate::model::book_info::form::BookForm;
use ratatui_image::{
    picker::Picker,
    thread::{ResizeResponse, ThreadProtocol},
};

pub const MIN_RATING: u8 = 0; // haven't read yet
pub const MAX_RATING: u8 = 5;
pub const DEFAULT_RATING: u8 = 0;

pub struct BookInfoState {
    pub mode: BookInfoMode,
    pub form: BookForm,
    pub image_picker: Picker,
    pub cover: CoverStatus,
}

impl BookInfoState {
    pub fn new(image_picker: Picker) -> Self {
        Self {
            mode: BookInfoMode::View,
            form: BookForm::default(),
            image_picker,
            cover: CoverStatus::None,
        }
    }

    pub fn handle_cover_response(&mut self, res: ResizeResponse) {
        if let CoverStatus::Ready(protocol) = &mut self.cover {
            let _ = protocol.update_resized_protocol(res);
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum BookInfoMode {
    View,
    Add,
    Edit,
}

pub enum CoverStatus {
    None,
    Loading,
    Ready(ThreadProtocol),
}
