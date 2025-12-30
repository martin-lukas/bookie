mod event;
mod image_util;
mod logging;
mod model;
mod view;

use crate::{
    event::{app_event::AppEvent, handle_key, spawn_input_thread},
    logging::setup_logger,
    model::{running_state::RunningState, Model},
    view::view,
};
use ratatui::DefaultTerminal;
use std::sync::mpsc;

fn main() -> color_eyre::Result<()> {
    setup_logger()?;
    color_eyre::install()?;

    let terminal = ratatui::init();
    let result = run(terminal);

    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
    let (event_tx, event_rx) = mpsc::channel::<AppEvent>();
    let mut model = Model::load(event_tx.clone());

    spawn_input_thread(event_tx.clone());

    // Initial render
    terminal.draw(|frame| view(&mut model, frame))?;

    while model.running_state != RunningState::Done {
        match event_rx.recv()? {
            AppEvent::Key(key) => {
                if let Some(msg) = handle_key(&model, key) {
                    let mut current = Some(msg);
                    while let Some(m) = current {
                        current = model.update(m);
                    }
                }
            }
            AppEvent::CoverReady(res) => {
                model.book_info.handle_cover_response(res);
            }
            AppEvent::Resize => {}
        }
        terminal.draw(|frame| view(&mut model, frame))?;
    }

    Ok(())
}
