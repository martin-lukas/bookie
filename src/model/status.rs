#[derive(Clone, Default)]
pub struct State {
    pub mode: Mode,
}

impl State {
    pub fn new() -> Self {
        Self { mode: Mode::Ok }
    }
}

#[derive(Clone, Default)]
pub enum Mode {
    #[default]
    Ok,
    Error(String),
    ConfirmDeleteBook,
}
