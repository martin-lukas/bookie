use crossterm::cursor::SetCursorStyle;
use crossterm::{
    cursor::{Hide, Show},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    io::{self, stderr, stdout, Write},
    panic,
};

pub fn install_panic_hook() {
    let default_hook = panic::take_hook();

    panic::set_hook(Box::new(move |panic_info| {
        let _ = disable_raw_mode();
        let _ = execute!(
            stderr(),
            SetCursorStyle::BlinkingBlock,
            LeaveAlternateScreen,
            Show
        );
        let _ = stderr().flush();

        default_hook(panic_info);
    }));
}

pub struct TerminalGuard;

impl TerminalGuard {
    pub fn enter() -> io::Result<Self> {
        enable_raw_mode()?;
        execute!(stdout(), EnterAlternateScreen, Hide)?;
        Ok(Self)
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = execute!(
            stdout(),
            SetCursorStyle::BlinkingBlock,
            LeaveAlternateScreen,
            Show
        );
        let _ = disable_raw_mode();
    }
}
