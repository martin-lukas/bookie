use ratatui::widgets::TableState;

#[derive(Clone, Default)]
pub struct State {
    pub table_state: TableState,
}

impl State {
    pub fn new(selected: Option<usize>) -> Self {
        let mut table_state = TableState::default();
        table_state.select(selected);
        Self { table_state }
    }

    pub fn selected(&self) -> Option<usize> {
        self.table_state.selected()
    }

    pub fn select_next(&mut self) {
        self.table_state.select_next();
    }

    pub fn select_previous(&mut self) {
        self.table_state.select_previous();
    }

    pub fn select(&mut self, index: Option<usize>) {
        self.table_state.select(index);
    }
}
