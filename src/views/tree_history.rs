use std::sync::{Arc, Mutex};

use crossterm::event::KeyCode;
use ratatui::{Frame, layout::Alignment, prelude::Rect, widgets::Block};

use crate::{persistance, widgets::bonsai::BonsaiWidget};

pub struct TreeHistoryView {
    page: u64,
    trees: Vec<BonsaiWidget>,
}

impl TreeHistoryView {
    pub fn new() -> Self {
        let mut trees: Vec<BonsaiWidget> = persistance::get_trees()
            .expect("Can't read history")
            .iter()
            .map(|b| BonsaiWidget::new(b.config, b.seed))
            .collect();
        for t in &mut trees {
            t.set_growth(100.0);
        }
        Self { page: 0, trees }
    }

    pub fn render(&mut self, frame: &mut Frame, input_events: Arc<Mutex<Vec<KeyCode>>>) {
        let (t_width, t_height) = crossterm::terminal::size().unwrap();

        self.handle_input(input_events);

        frame.render_widget(
            Block::bordered()
                .title("Bonsai History")
                .title_alignment(Alignment::Center)
                .title_bottom(format!(
                    "page {} of {}",
                    self.page + 1,
                    (self.trees.len() as f64 / 2.0).ceil()
                )),
            Rect::new(0, 0, t_width, t_height),
        );

        for (index, widget) in &mut self.trees[(self.page * 2) as usize
            ..=((self.page * 2) + 1) as usize
                - if (self.page * 2) as usize == (self.trees.len() - 1) {
                    self.trees.len() % 2
                } else {
                    0
                }]
            .iter()
            .enumerate()
        {
            frame.render_widget(
                widget.get(),
                Rect::new(
                    if index == 0 { 0 } else { t_width / 2 },
                    0,
                    t_width / 2,
                    t_height,
                ),
            );
        }
    }

    fn handle_input(&mut self, input_events: Arc<Mutex<Vec<KeyCode>>>) {
        input_events
            .lock()
            .unwrap()
            .retain(|key_code| match key_code {
                KeyCode::Right => {
                    if self.page < (self.trees.len() as f64 / 2.0).ceil() as u64 - 1 {
                        self.page += 1;
                    }
                    false
                }
                KeyCode::Left => {
                    if (self.page as i64 - 1) >= 0 {
                        self.page -= 1;
                    }
                    false
                }

                _ => false,
            })
    }
}
