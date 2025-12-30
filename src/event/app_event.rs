use ratatui::crossterm::event::KeyEvent;
use ratatui_image::thread::ResizeResponse;

pub enum AppEvent {
    Key(KeyEvent),
    CoverReady(ResizeResponse),
    Resize,
}
