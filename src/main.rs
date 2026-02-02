use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{DefaultTerminal, Frame};

use crate::views::{View, timer_view::TimerView, tree_history::TreeHistoryView};

mod persistance;
mod util;
mod views;
mod widgets;

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let mut app = App::new();
    let result = app.run(terminal);
    ratatui::restore();
    result
}

struct App {
    quit: bool,
    view: View,
    input_events: Arc<Mutex<Vec<KeyCode>>>,
}

impl App {
    fn new() -> Self {
        App {
            quit: false,
            view: View::Main,
            input_events: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn run(&mut self, mut terminal: DefaultTerminal) -> Result<()> {
        let mut tree_history_view = TreeHistoryView::new();
        let mut timer_view = TimerView::new();

        let input_events = Arc::clone(&self.input_events);
        thread::spawn(move || {
            loop {
                if let Event::Key(key) = event::read().unwrap() {
                    input_events.lock().unwrap().push(key.code);
                }
            }
        });

        loop {
            if self.quit {
                return Result::Ok(());
            }

            self.handle_global_input();

            let input_events = Arc::clone(&self.input_events);
            terminal.draw(|frame: &mut Frame| match self.view {
                View::Main => timer_view.render(frame, input_events),
                View::History => tree_history_view.render(frame, input_events),
            })?;

            thread::sleep(Duration::from_millis(50));
        }
    }

    fn handle_global_input(&mut self) {
        self.input_events
            .lock()
            .unwrap()
            .retain(|key_code| match key_code {
                KeyCode::Char('q') => {
                    self.quit = true;
                    false
                }
                KeyCode::Char('n') => {
                    self.view = View::History;
                    false
                }
                KeyCode::Char('m') => {
                    self.view = View::Main;
                    false
                }

                _ => true,
            });
    }
}
