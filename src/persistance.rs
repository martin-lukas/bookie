use crate::{app::App, book::Book, view::View};
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
    pub view: View,
    pub selected: usize,
}

impl SavedState {
    pub fn new(app: App) -> Self {
        Self {
            books: app.books,
            view: app.view,
            selected: app.selected,
        }
    }
}

pub fn load_state() -> io::Result<SavedState> {
    let data = fs::read_to_string(SAVED_STATE_PATH)?;

    let saved_state: SavedState = serde_json::from_str(&data).expect("Failed to parse JSON");
    Ok(saved_state)
}

pub fn save_state(app: App) -> io::Result<()> {
    let saved_state = SavedState::new(app);
    let json = serde_json::to_string_pretty(&saved_state).expect("Failed to serialize books");
    let mut file = File::create(SAVED_STATE_PATH)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}
