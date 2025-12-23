#[derive(Clone, Default)]
pub struct State {
    pub text: String,
    pub input: String,
    pub mode: Mode,
}

impl State {
    pub fn new() -> Self {
        Self {
            text: String::new(),
            input: String::new(),
            mode: Mode::Ok,
        }
    }
}

#[derive(Clone, Default)]
pub enum Mode {
    #[default]
    Ok,
    Error(String),
    ConfirmDeleteBook,
}
