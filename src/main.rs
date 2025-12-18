mod domain;
mod event;
mod exit;
mod logging;
mod persistance;
mod render;
mod util;

use crate::domain::layout::{Pane, Rect};
use crate::domain::view::View;
use crate::{
    domain::{app::App, layout::Layout},
    event::handle_event,
    exit::{install_panic_hook, TerminalGuard},
    render::render,
};
use crossterm::terminal;
use log::info;
use std::io;

fn main() -> io::Result<()> {
    install_panic_hook();
    let _terminal = TerminalGuard::enter()?;

    logging::setup_logger().expect("Failed to setup logger");
    info!("BOOKIE STARTED");

    let saved_state = persistance::load_state()?;

    let (width, height) = terminal::size()?;
    let layout = Layout {
        list: Pane::new(View::BookList, Rect::new(0, 0, width, height / 2 - 1), true),
        detail: Pane::new(
            View::BookDetail,
            Rect::new(0, height / 2, width, height - 1),
            false,
        ),
    };

    let mut app = App::new(saved_state, layout);

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


