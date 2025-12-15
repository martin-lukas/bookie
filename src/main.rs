mod app;
mod book;
mod event_handling;
mod exit;
mod logging;
mod persistance;
mod rendering;
mod util;
mod view;

use crate::{
    app::App, event_handling::handle_event, exit::install_panic_hook, exit::TerminalGuard,
    rendering::Renderer,
};
use log::info;
use std::io;

fn main() -> io::Result<()> {
    install_panic_hook();
    let _terminal = TerminalGuard::enter()?;

    logging::setup_logger().expect("Failed to setup logger");
    info!("BOOKIE STARTED");

    let saved_state = persistance::load_state()?;
    let mut app = App::new(saved_state);

    loop {
        Renderer::render(&app)?;
        handle_event(&mut app)?;
        if app.should_quit {
            break;
        }
    }

    info!("BOOKIE EXITING");
    persistance::save_state(app)?;

    Ok(())
}
