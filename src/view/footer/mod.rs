mod status;

use crate::{model::model::Model, view::footer::status::render_status};
use ratatui::{layout::Rect, Frame};

pub fn render_footer(model: &Model, frame: &mut Frame, area: Rect) {
    render_status(model, frame, area);
}
