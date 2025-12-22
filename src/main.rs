mod event;
mod logging;
mod model;
mod persistance;
mod util;
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
    let saved_state = persistance::load_state()?;
    let mut model = Model::from(saved_state);

    while model.running_state != RunningState::Done {
        terminal.draw(|frame| view(&mut model, frame))?;
        let mut current_msg = handle_event(&model)?;
        while current_msg.is_some() {
            current_msg = model.update(current_msg.unwrap());
        }
    }

    persistance::save_state(model)?;
    Ok(())
}
