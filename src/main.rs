mod domain;
mod event;
mod persistance;
mod view;

use crate::{
    domain::model::{Model, RunningState},
    event::{handle_event, update},
    view::view,
};
use ratatui::DefaultTerminal;

fn main() -> color_eyre::Result<()> {
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
            current_msg = update(&mut model, current_msg.unwrap());
        }
    }

    persistance::save_state(model)?;
    Ok(())
}
