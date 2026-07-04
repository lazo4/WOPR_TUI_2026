use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Clear, Paragraph},
};

use crate::state::AppState;

const BOOT_LINES: &[&str] = &[
    "LOGON:",
    "",
    "IDENTIFICATION NOT RECOGNIZED BY SYSTEM",
    "--CONNECTION TERMINATED--",
    "",
    "LOGON: Joshua",
    "",
    "GREETINGS, PROFESSOR FALKEN.",
    "",
    "HOW ARE YOU FEELING TODAY?",
    "",
    "EXCELLENT. IT'S BEEN A LONG TIME.",
    "CAN YOU EXPLAIN THE REMOVAL OF YOUR USER ACCOUNT",
    "ON 6/23/73?",
    "",
    "SHALL WE PLAY A GAME?",
];

pub fn render_splash(frame: &mut Frame, area: Rect, state: &AppState) {
    frame.render_widget(Clear, area);

    let green = Style::default().fg(Color::Green);
    let cursor_visible = state.tick_count % 30 < 15;

    let mut lines: Vec<Line> = Vec::new();
    lines.push(Line::from(""));
    lines.push(Line::from(""));

    for (i, &line_text) in BOOT_LINES.iter().enumerate() {
        if i > state.boot_phase {
            break;
        }
        if i < state.boot_phase {
            // fully typed
            lines.push(Line::from(Span::styled(format!("    {}", line_text), green)));
        } else {
            // currently typing
            if line_text.is_empty() {
                lines.push(Line::from(""));
            } else {
                let visible: String = line_text.chars().take(state.boot_char_index).collect();
                let cursor = if cursor_visible && state.boot_char_index < line_text.len() { "▌" } else { "" };
                lines.push(Line::from(Span::styled(format!("    {}{}", visible, cursor), green)));
            }
        }
    }

    // blinking cursor on empty line after all lines typed
    if state.boot_phase >= BOOT_LINES.len() && cursor_visible {
        lines.push(Line::from(Span::styled("    ▌", green)));
    }

    frame.render_widget(Paragraph::new(lines), area);
}
