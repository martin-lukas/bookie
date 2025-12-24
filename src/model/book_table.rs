use ratatui::widgets::{ScrollbarState, TableState};

#[derive(Clone, Default)]
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

    pub fn select_next(&mut self) {
        self.table_state.select_next();
        self.sync_scrollbar_position();
    }

    pub fn select_previous(&mut self) {
        self.table_state.select_previous();
        self.sync_scrollbar_position();
    }

    pub fn select(&mut self, index: Option<usize>) {
        self.table_state.select(index);
        self.sync_scrollbar_position();
    }

    pub fn update_scrollbar_length(&mut self, length: usize) {
        self.scrollbar_state = self.scrollbar_state.content_length(length);
    }

    fn sync_scrollbar_position(&mut self) {
        self.scrollbar_state = self
            .scrollbar_state
            .position(self.table_state.selected().unwrap_or(0));
    }
}
