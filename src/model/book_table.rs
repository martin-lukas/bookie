use ratatui::widgets::TableState;

#[derive(Clone)]
pub struct State {
    pub table_state: TableState,
}

impl State {
    pub fn new(selected: usize) -> Self {
        let mut table_state = TableState::default();
        table_state.select(Some(selected));
        Self { table_state }
    }

    pub fn selected_unsafe(&self) -> usize {
        self.table_state.selected().unwrap()
    }

    pub fn select_next(&mut self) {
        self.table_state.select_next();
    }

    pub fn select_previous(&mut self) {
        self.table_state.select_previous();
    }
}
