use crossterm::event::KeyEvent;
use tokio::sync::mpsc;

// ponytail: placeholder — T6.1 expands with all game state transitions
#[derive(Debug, Clone)]
pub enum GameEvent {}

#[derive(Debug)]
pub enum AppEvent {
    Input(KeyEvent),
    Tick,
    Resize(u16, u16),
    GameEvent(GameEvent),
    Quit,
}

pub fn create_channel() -> (mpsc::Sender<AppEvent>, mpsc::Receiver<AppEvent>) {
    mpsc::channel(256)
}
