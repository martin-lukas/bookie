#[derive(Default)]
pub struct State {
    pub mode: StatusMode,
}

impl State {
    pub fn new() -> Self {
        Self { mode: StatusMode::Ok }
    }
}

#[derive(Default)]
pub enum StatusMode {
    #[default]
    Ok,
    Error(String),
    ConfirmDeleteBook,
}
