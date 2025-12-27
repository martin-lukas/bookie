use ratatui_image::picker::Picker;

pub fn create_picker() -> Picker {
    Picker::from_query_stdio().unwrap_or_else(|_| Picker::from_fontsize((8, 12)))
}
