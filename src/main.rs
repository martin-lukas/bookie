mod app;
mod book;
mod event_handling;
mod logging;
mod persistance;
mod rendering;
mod view;

use crate::{app::App, event_handling::handle_event, rendering::Renderer};
use log::info;
use std::io;

fn main() -> io::Result<()> {
    logging::setup_logger().expect("Failed to setup logger");
    info!("BOOKIE STARTED");

    Renderer::init_screen()?;

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
    Renderer::exit_screen()?;
    persistance::save_state(app)?;

    Ok(())
}
