use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, Clear, Paragraph},
};

use crate::mode::Mode;
use crate::state::AppState;
use crate::view::ViewState;

pub fn render(frame: &mut Frame, view: &ViewState, state: &AppState) {
    render_status_bar(frame, view, state);
    render_content(frame, view, state);
    render_input_bar(frame, view);
    if state.show_help {
        render_help(frame, view);
    }
}

fn defcon_color(level: u8) -> Color {
    match level {
        5 => Color::Green,
        4 => Color::Cyan,
        3 => Color::Yellow,
        2 => Color::Magenta,
        _ => Color::Red,
    }
}

fn render_status_bar(frame: &mut Frame, view: &ViewState, state: &AppState) {
    let text = format!(
        " DEFCON {} │ {} │ TICK {:06}",
        state.defcon_level, state.mode, state.tick_count
    );
    frame.render_widget(
        Paragraph::new(text).style(Style::default().fg(defcon_color(state.defcon_level))),
        view.status_bar,
    );
}

fn render_content(frame: &mut Frame, view: &ViewState, state: &AppState) {
    match &state.mode {
        Mode::MainMap => render_placeholder(frame, view.content, "MAIN MAP"),
        Mode::Comms => render_placeholder(frame, view.content, "COMMS"),
        Mode::Settings => render_placeholder(frame, view.content, "SETTINGS"),
        Mode::Scenario => render_placeholder(frame, view.content, "SCENARIO"),
        Mode::Defcon => render_placeholder(frame, view.content, "DEFCON"),
    }
}

fn render_placeholder(frame: &mut Frame, area: Rect, name: &str) {
    let block = Block::default().borders(Borders::ALL).title(format!(" {name} "));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    // vertically center the mode label
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Length(1), Constraint::Min(0)])
        .split(inner);
    frame.render_widget(
        Paragraph::new(name)
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::Green)),
        rows[1],
    );
}

fn render_input_bar(frame: &mut Frame, view: &ViewState) {
    frame.render_widget(
        Paragraph::new(" [Tab] Next  [Shift+Tab] Prev  [?] Help  [q] Quit")
            .style(Style::default().fg(Color::DarkGray)),
        view.input_bar,
    );
}

fn render_help(frame: &mut Frame, view: &ViewState) {
    let popup = centered_rect(50, 40, view.content);
    let block = Block::default().borders(Borders::ALL).title(" HELP ");
    let inner = block.inner(popup);
    frame.render_widget(Clear, popup);
    frame.render_widget(block, popup);
    frame.render_widget(
        Paragraph::new(vec![
            Line::from("  [Tab]         Next mode"),
            Line::from("  [Shift+Tab]   Prev mode"),
            Line::from("  [?]           Close help"),
            Line::from("  [q]           Quit"),
        ]),
        inner,
    );
}

fn centered_rect(pct_x: u16, pct_y: u16, r: Rect) -> Rect {
    let vert = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - pct_y) / 2),
            Constraint::Percentage(pct_y),
            Constraint::Percentage((100 - pct_y) / 2),
        ])
        .split(r);
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - pct_x) / 2),
            Constraint::Percentage(pct_x),
            Constraint::Percentage((100 - pct_x) / 2),
        ])
        .split(vert[1])[1]
}
