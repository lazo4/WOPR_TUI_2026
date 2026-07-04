use crate::mode::Mode;

pub struct AppState {
    pub mode: Mode,
    pub defcon_level: u8,
    pub tick_count: u64,
    pub terminal_size: (u16, u16),
    pub messages: Vec<String>,
    pub game_active: bool,
    pub show_help: bool,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            mode: Mode::MainMap,
            defcon_level: 5,
            tick_count: 0,
            terminal_size: (80, 24),
            messages: Vec::new(),
            game_active: false,
            show_help: false,
        }
    }

    pub fn tick(&mut self) {
        self.tick_count = self.tick_count.saturating_add(1);
    }

    pub fn resize(&mut self, w: u16, h: u16) {
        self.terminal_size = (w, h);
    }
}
