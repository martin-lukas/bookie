mod app;
mod logging;

use crate::app::App;
use color_eyre::Result;

fn main() -> Result<()> {
    logging::setup_logger().expect("Failed to setup logger");

    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new().run(terminal);
    ratatui::restore();
    result
}
