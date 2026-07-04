use ratatui::layout::{Constraint, Direction, Layout, Rect};

use crate::state::AppState;

pub struct ViewState {
    pub status_bar: Rect,
    pub content: Rect,
    pub input_bar: Rect,
}

pub fn compute_view(_state: &AppState, area: Rect) -> ViewState {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .split(area);

    ViewState {
        status_bar: chunks[0],
        content: chunks[1],
        input_bar: chunks[2],
    }
}
