use std::sync::{Arc, Mutex};

use crossterm::{event::KeyCode, terminal};
use rand::random;
use ratatui::{
    Frame,
    layout::{Constraint, Flex, Layout},
    widgets::Block,
};
use rbonsai::bonsai::TreeConfig;

use crate::{
    persistance::{self, TreeStore},
    widgets::{
        bonsai::BonsaiWidget, networking_widget::NetworkingWidget, timer_widget::TimerWidget,
    },
};

pub struct TimerView {
    timer_widget: TimerWidget,
    bonsai_widget: BonsaiWidget,
    networking_widget: NetworkingWidget,
    tree_config: Option<TreeConfig>,
    tree_seed: Option<u64>,
}

impl TimerView {
    pub fn new() -> Self {
        Self {
            timer_widget: TimerWidget::new(),
            // placeholder tree only for the pot
            bonsai_widget: BonsaiWidget::new(
                TreeConfig {
                    max_x: terminal::size().unwrap().0 / 2,
                    max_y: terminal::size().unwrap().1,
                    life: 30,
                    multiplier: 3,
                },
                0,
            ),
            networking_widget: NetworkingWidget::new(),
            tree_config: None,
            tree_seed: None,
        }
    }

    pub fn render(&mut self, frame: &mut Frame, input_events: Arc<Mutex<Vec<KeyCode>>>) {
        self.handle_input(input_events);

        self.timer_widget.update();

        // timer is active?
        if self.timer_widget.timer_startpoint.is_some() {
            let elapsed_time = self
                .timer_widget
                .timer_startpoint
                .unwrap()
                .elapsed()
                .as_millis() as f32;
            let goal = self.timer_widget.goal_duration.as_millis() as f32;

            if elapsed_time >= goal {
                self.timer_widget.timer_startpoint = None;
                persistance::add_tree(TreeStore {
                    config: self.tree_config.unwrap(),
                    seed: self.tree_seed.unwrap(),
                    withered: false,
                })
                .expect("Couldn't save tree to history.");
            } else {
                self.bonsai_widget
                    .set_growth((elapsed_time / goal) * 100f32);
            }
        }

        let horizontal =
            Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                .flex(Flex::Center);
        let [tree_area, timer_area] = horizontal.areas(frame.area());

        // timer at upper right corner
        let vertical =
            Layout::vertical([Constraint::Fill(2), Constraint::Fill(1)]).flex(Flex::Center);
        let [timer_area, networking_area] = vertical.areas(timer_area);
        self.timer_widget.render(frame, timer_area);

        // networking at bottom right corner
        self.networking_widget.render(frame, networking_area);

        // bonsai with bordered block
        let block = Block::bordered();
        frame.render_widget(block, tree_area);

        frame.render_widget(self.bonsai_widget.get(), tree_area);
    }

    fn handle_input(&mut self, input_events: Arc<Mutex<Vec<KeyCode>>>) {
        input_events
            .lock()
            .unwrap()
            .retain(|key_code| match key_code {
                KeyCode::Char(' ') => {
                    self.timer_widget.toggle();
                    self.tree_seed = Some(random());
                    self.tree_config = Some(TreeConfig {
                        max_x: terminal::size().unwrap().0 / 2,
                        max_y: terminal::size().unwrap().1,
                        life: 30,
                        multiplier: 3,
                    });
                    self.bonsai_widget =
                        BonsaiWidget::new(self.tree_config.unwrap(), self.tree_seed.unwrap());
                    false
                }
                KeyCode::Left | KeyCode::Char('h') => {
                    self.timer_widget.move_selector_left();
                    false
                }
                KeyCode::Right | KeyCode::Char('l') => {
                    self.timer_widget.move_selector_right();
                    false
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    self.timer_widget.modify_goal_time(1);
                    false
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    self.timer_widget.modify_goal_time(-1);
                    false
                }

                _ => false,
            });
    }
}
