mod event;
mod image_util;
mod logging;
mod model;
mod view;

use crate::{
    event::handle_event,
    logging::setup_logger,
    model::model::{Model, RunningState},
    view::view,
};
use ratatui::DefaultTerminal;

fn main() -> color_eyre::Result<()> {
    setup_logger()?;
    color_eyre::install()?;

    let terminal = ratatui::init();
    let result = run(terminal);

    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
    let mut model = Model::load();

    while model.running_state != RunningState::Done {
        terminal.draw(|frame| view(&mut model, frame))?;
        let mut current_msg = handle_event(&model)?;
        while current_msg.is_some() {
            current_msg = model.update(current_msg.unwrap());
        }
    }

    Ok(())
}
