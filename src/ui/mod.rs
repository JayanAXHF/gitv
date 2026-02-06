use std::time::Duration;

use crate::{
    errors::AppError,
    ui::components::{Component, search_bar::TextSearch},
};
use crossterm::event::EventStream;
use futures::{StreamExt, future::FutureExt};
use ratatui::{crossterm, prelude::*, widgets::Block};
use tokio::{select, sync::mpsc::Sender};
use tokio_util::sync::CancellationToken;

const TICK_RATE: std::time::Duration = std::time::Duration::from_millis(100);
const FPS: usize = 60;

pub async fn run() -> Result<(), AppError> {
    let mut terminal = ratatui::init();
    let (action_tx, action_rx) = tokio::sync::mpsc::channel(100);
    let mut app = App::new(action_tx, action_rx);
    app.run(&mut terminal).await?;
    ratatui::restore();
    Ok(())
}

struct App {
    action_tx: tokio::sync::mpsc::Sender<Action>,
    action_rx: tokio::sync::mpsc::Receiver<Action>,
    cancel_action: CancellationToken,
    components: Vec<Box<dyn Component>>,
}

impl App {
    pub fn new(action_tx: Sender<Action>, action_rx: tokio::sync::mpsc::Receiver<Action>) -> Self {
        let text_search = TextSearch::default();
        Self {
            action_tx,
            action_rx,
            cancel_action: Default::default(),
            components: vec![Box::new(text_search)],
        }
    }
    pub async fn run(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<impl std::io::Write>>,
    ) -> Result<(), AppError> {
        let ctok = self.cancel_action.clone();

        let action_tx = self.action_tx.clone();

        tokio::spawn(async move {
            let mut tick_interval = tokio::time::interval(TICK_RATE);
            let mut frame_interval =
                tokio::time::interval(Duration::from_secs_f64(1.0 / FPS as f64));
            let mut event_stream = EventStream::new();

            loop {
                let event = select! {
                    _ = ctok.cancelled() => break,
                    _ = tick_interval.tick() => Action::Tick,
                    _ = frame_interval.tick() => {
                        Action::Render
                    },
                    kevent = event_stream.next().fuse() => {
                        match kevent {
                            Some(Ok(kevent)) => Action::AppEvent(kevent),
                            Some(Err(..)) => Action::None,
                            None => break,
                        }
                    }
                };
                if action_tx.send(event).await.is_err() {
                    break;
                }
            }
            Ok::<(), AppError>(())
        });

        let ctok = self.cancel_action.clone();
        loop {
            let action = self.action_rx.recv().await;
            match action {
                Some(Action::None) => {}
                Some(Action::Tick) => {
                    terminal.draw(|f| {
                        let layout = layout::Layout::new(f.area());
                        let buf = f.buffer_mut();
                        let areas = layout.areas();
                        for area in areas {
                            let w = Block::bordered()
                                .border_type(ratatui::widgets::BorderType::Rounded);
                            w.render(area, buf);
                        }
                        for component in self.components.iter_mut() {
                            component.render(layout, buf);
                        }
                    })?;
                }
                Some(Action::Render) => {}
                Some(Action::AppEvent(event)) => {
                    self.handle_event(event).await?;
                }
                Some(Action::Quit) | None => {
                    ctok.cancel();
                    break;
                }
            }
            if self.cancel_action.is_cancelled() {
                break;
            }
        }

        Ok(())
    }
    async fn handle_event(&mut self, event: crossterm::event::Event) -> Result<(), AppError> {
        match event {
            crossterm::event::Event::Key(key) => {
                self.handle_key(key).await?;
                for component in self.components.iter_mut() {
                    component.handle_event(Action::AppEvent(event.clone()));
                }
            }
            _ => {}
        }
        Ok(())
    }
    async fn handle_key(&mut self, key: crossterm::event::KeyEvent) -> Result<(), AppError> {
        match key.code {
            crossterm::event::KeyCode::Char('q') => {
                self.cancel_action.cancel();
            }
            _ => {}
        }

        Ok(())
    }
}

#[non_exhaustive]
pub enum Action {
    None,
    Tick,
    Render,
    Quit,
    AppEvent(crossterm::event::Event),
}

pub mod components;
pub mod keystrokes;
pub mod layout;
pub mod theme;
