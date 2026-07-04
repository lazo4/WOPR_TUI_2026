use std::io::{self, Stdout};
use std::time::Duration;

use crossterm::event::{Event as CEvent, EventStream, KeyCode, KeyEventKind};
use futures::StreamExt;
use ratatui::{backend::CrosstermBackend, Terminal};
use tokio::sync::mpsc;

use crate::event::{create_channel, AppEvent};
use crate::state::AppState;
use crate::ui::render;
use crate::view::compute_view;

pub struct App {
    state: AppState,
    tx: mpsc::Sender<AppEvent>,
    rx: mpsc::Receiver<AppEvent>,
}

impl App {
    pub fn new() -> Self {
        let (tx, rx) = create_channel();
        Self { state: AppState::new(), tx, rx }
    }

    pub async fn run(mut self) -> io::Result<()> {
        let mut terminal = crate::terminal::init_terminal()?;
        let result = self.event_loop(&mut terminal).await;
        crate::terminal::restore_terminal();
        result
    }

    async fn event_loop(&mut self, terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> io::Result<()> {
        let mut ticker = tokio::time::interval(Duration::from_millis(16));
        let mut events = EventStream::new();

        loop {
            tokio::select! {
                _ = ticker.tick() => {
                    // drain up to 64 queued events before rendering
                    let mut drained = 0;
                    while drained < 64 {
                        match self.rx.try_recv() {
                            Ok(AppEvent::Quit) => return Ok(()),
                            Ok(ev) => { self.handle_event(ev); drained += 1; }
                            Err(_) => break,
                        }
                    }
                    self.state.tick();
                    terminal.draw(|f| {
                        let view = compute_view(&self.state, f.area());
                        render(f, &view, &self.state);
                    })?;
                }
                maybe = events.next() => {
                    match maybe {
                        Some(Ok(CEvent::Key(k))) if k.kind == KeyEventKind::Press => {
                            let ev = if k.code == KeyCode::Char('q') {
                                AppEvent::Quit
                            } else {
                                AppEvent::Input(k)
                            };
                            let _ = self.tx.send(ev).await;
                        }
                        Some(Ok(CEvent::Resize(w, h))) => {
                            let _ = self.tx.send(AppEvent::Resize(w, h)).await;
                        }
                        None => return Ok(()),
                        _ => {}
                    }
                }
            }
        }
    }

    fn handle_event(&mut self, event: AppEvent) {
        match event {
            AppEvent::Tick => self.state.tick(),
            AppEvent::Resize(w, h) => self.state.resize(w, h),
            AppEvent::Input(_) if self.state.show_help => self.state.show_help = false,
            AppEvent::Input(k) => match k.code {
                KeyCode::Tab => self.state.mode = self.state.mode.next(),
                KeyCode::BackTab => self.state.mode = self.state.mode.prev(),
                KeyCode::Char('?') => self.state.show_help = true,
                _ => {}
            },
            AppEvent::GameEvent(_) => {} // ponytail: T6.1 adds game event handling
            AppEvent::Quit => {}
        }
    }
}
