use crate::domain::{book::Book, model::Model};
use serde::{Deserialize, Serialize};
use serde_json;
use std::{
    fs::{self, File},
    io::{self, Write},
};

const SAVED_STATE_PATH: &str = "saved_state.json";

#[derive(Debug, Serialize, Deserialize)]
pub struct SavedState {
    pub books: Vec<Book>,
    pub selected: usize,
}

impl SavedState {
    pub fn new(model: Model) -> Self {
        Self {
            books: model.books,
            selected: model.selected,
        }
    }

    pub fn empty() -> Self {
        Self {
            books: vec![],
            selected: 0,
        }
    }
}

pub fn load_state() -> color_eyre::Result<SavedState> {
    let data = fs::read_to_string(SAVED_STATE_PATH)?;

    let saved_state: SavedState = serde_json::from_str(&data).expect("Failed to parse JSON");
    Ok(saved_state)
}

pub fn save_state(model: Model) -> color_eyre::Result<()> {
    let saved_state = SavedState::new(model);
    let json = serde_json::to_string_pretty(&saved_state).expect("Failed to serialize books");
    let mut file = File::create(SAVED_STATE_PATH)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}
