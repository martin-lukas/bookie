use crate::View;
use crossterm::event::{KeyCode, KeyEvent};

pub trait EventHandler {
    fn handle(&mut self, key_code: &KeyEvent) -> HandleResult;
}

pub enum HandleResult {
    Handled,
    Ignored,
    Quit,
}

impl<'a> EventHandler for View<'a> {
    fn handle(&mut self, key_event: &KeyEvent) -> HandleResult {
        match key_event.code {
            KeyCode::Up => {
                if self.selected > 0 {
                    self.selected -= 1;
                }
                HandleResult::Handled
            }
            KeyCode::Down => {
                if self.selected + 1 < self.books.len() {
                    self.selected += 1;
                }
                HandleResult::Handled
            }
            _ => HandleResult::Ignored,
        }
    }
}
