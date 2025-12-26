use ratatui::widgets::{ScrollbarState, TableState};

#[derive(Default)]
pub struct State {
    pub table_state: TableState,
    pub scrollbar_state: ScrollbarState,
}

impl State {
    pub fn new(size: usize, selected: Option<usize>) -> Self {
        let mut table_state = TableState::default();
        table_state.select(selected);
        let selected = table_state.selected().unwrap_or(0);
        Self {
            table_state,
            scrollbar_state: ScrollbarState::new(size).position(selected),
        }
    }

    pub fn selected(&self) -> Option<usize> {
        self.table_state.selected()
    }
    
    pub fn update_scrollbar_length(&mut self, length: usize) {
        self.scrollbar_state = self.scrollbar_state.content_length(length);
    }

    pub fn sync_scrollbar_position(&mut self) {
        self.scrollbar_state = self
            .scrollbar_state
            .position(self.table_state.selected().unwrap_or(0));
    }
}
