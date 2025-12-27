pub mod form;

use ratatui_image::{
    picker::Picker,
    thread::{ResizeResponse, ThreadProtocol},
};
use std::sync::mpsc::Receiver;

use crate::model::book_info::form::BookForm;

pub const MIN_RATING: u8 = 0; // haven't read yet
pub const MAX_RATING: u8 = 5;
pub const DEFAULT_RATING: u8 = 0;

pub struct BookInfoState {
    pub mode: BookInfoMode,
    pub form: BookForm,
    pub image_picker: Picker,
    pub cover_rx: Option<Receiver<ResizeResponse>>,
    pub cover: CoverStatus,
}

impl BookInfoState {
    pub fn new(image_picker: Picker) -> Self {
        Self {
            mode: BookInfoMode::View,
            form: BookForm::default(),
            image_picker,
            cover_rx: None,
            cover: CoverStatus::None,
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
