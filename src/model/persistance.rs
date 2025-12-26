use crate::model::{book::Book, model::Model};
use serde::{Deserialize, Serialize};
use serde_json;
use std::{
    fs::{self, File},
    io::Write,
};

const SAVED_STATE_PATH: &str = "bookie-state.json";

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SavedState {
    pub books: Vec<Book>,
    pub selected: Option<usize>,
}

impl SavedState {
    pub fn from(model: &Model) -> Self {
        Self {
            books: model.books.iter().map(|b| b.clone()).collect(),
            selected: model.book_table.selected(),
        }
    }
}

pub fn load() -> color_eyre::Result<SavedState> {
    match fs::read_to_string(SAVED_STATE_PATH) {
        Ok(data) => {
            let saved_state = serde_json::from_str(&data).expect("Failed to parse state from JSON");
            Ok(saved_state)
        }
        Err(_) => {
            let saved_state = SavedState::default();
            save_state(&Model::from(saved_state.clone()))?;
            Ok(saved_state)
        }
    }
}

pub fn save_state(model: &Model) -> color_eyre::Result<()> {
    let saved_state = SavedState::from(&model);
    let json = serde_json::to_string_pretty(&saved_state).expect("Failed to save state into JSON");
    let mut file = File::create(SAVED_STATE_PATH)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}
