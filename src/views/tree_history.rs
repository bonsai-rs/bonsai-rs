use std::time::Duration;

use crossterm::event::{self, KeyCode};
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

    pub fn render(&mut self, frame: &mut Frame) {
        let (t_width, t_height) = crossterm::terminal::size().unwrap();

        if event::poll(Duration::from_millis(500)).unwrap() {
            let e = event::read().unwrap();
            match e.as_key_event().unwrap().code {
                KeyCode::Right => {
                    if self.page < (self.trees.len() as f64 / 2.0).ceil() as u64 - 1 {
                        self.page += 1;
                    }
                }
                KeyCode::Left => 'left: {
                    if !((self.page as i64 - 1) >= 0) {
                        break 'left;
                    }
                    self.page -= 1
                }
                _ => (),
            }
        }

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
                - if (self.page * 2) as usize == (self.trees.len() - 1) as usize {
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
}
