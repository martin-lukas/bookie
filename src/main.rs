mod domain;
mod event;
mod exit;
mod logging;
mod persistance;
mod render;
mod util;

use crate::{
    domain::{
        model::Model,
        layout::{Layout, Pane, Rect},
    },
    event::handle_event,
    exit::{install_panic_hook, TerminalGuard},
    render::render,
};
use crossterm::terminal;
use log::info;
use std::io;

const PANE_BORDER: u16 = 1;
const RIGHT_PANE_WIDTH: u16 = 30;

fn main() -> io::Result<()> {
    install_panic_hook();
    let _terminal = TerminalGuard::enter()?;

    logging::setup_logger().expect("Failed to setup logger");
    info!("BOOKIE STARTED");

    let saved_state = persistance::load_state()?;

    let (cols, rows) = terminal::size()?;
    let layout = Layout {
        cols,
        rows,
        top: Rect::new(
            PANE_BORDER,
            PANE_BORDER,
            cols - RIGHT_PANE_WIDTH - PANE_BORDER,
            rows / 2,
        ),
        bottom: Rect::new(
            PANE_BORDER,
            rows / 2 + PANE_BORDER,
            cols - RIGHT_PANE_WIDTH - PANE_BORDER,
            rows / 2,
        ),
        right: Rect::new(
            cols - RIGHT_PANE_WIDTH,
            PANE_BORDER,
            RIGHT_PANE_WIDTH,
            rows - PANE_BORDER,
        ),
        focused: Pane::Top,
    };

    let mut app = Model::new(saved_state, layout);

    loop {
        render(&app)?;
        handle_event(&mut app)?;
        if app.should_quit {
            break;
        }
    }

    info!("BOOKIE EXITING");
    persistance::save_state(app)?;

    Ok(())
}
