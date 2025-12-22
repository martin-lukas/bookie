mod title;

use crate::model::model::Model;
use crate::view::header::title::render_title;
use ratatui::{layout::Rect, Frame};

pub fn render_header(_: &Model, frame: &mut Frame, area: Rect) {
    render_title(frame, area);
}
