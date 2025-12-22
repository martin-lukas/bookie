#[derive(Clone)]
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

#[derive(Clone)]
pub enum Mode {
    Ok,
    Error(String),
    ConfirmDeleteBook,
}
