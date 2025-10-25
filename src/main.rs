use std::time::Duration;

use color_eyre::Result;
use crossterm::{
    event::{self, KeyCode},
    terminal,
};
use rand::random;
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Flex, Layout, Rect},
    widgets::Block,
};
use rbonsai::bonsai::TreeConfig;
use widgets::timer_widget::TimerWidget;

use crate::views::tree_history::{self, TreeHistoryView};
use crate::{
    persistance::TreeStore, widgets::bonsai::BonsaiWidget,
    widgets::networking_widget::NetworkingWidget,
};

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

enum View {
    Main,
    History,
}

struct App {
    timer_widget: TimerWidget,
    bonsai_widget: BonsaiWidget,
    networking_widget: NetworkingWidget,
    quit: bool,
    config: Option<TreeConfig>,
    seed: Option<u64>,
    view: View,
}

impl App {
    fn new() -> Self {
        App {
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
            quit: false,
            config: None,
            seed: None,
            view: View::Main,
        }
    }

    pub fn run(&mut self, mut terminal: DefaultTerminal) -> Result<()> {
        let mut thv = TreeHistoryView::new();
        loop {
            if self.quit {
                return Result::Ok(());
            }
            self.handle_input();
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
                        config: self.config.unwrap(),
                        seed: self.seed.unwrap(),
                        withered: false,
                    })?;
                } else {
                    self.bonsai_widget
                        .set_growth((elapsed_time / goal) * 100f32);
                }
            }

            terminal.draw(|frame: &mut Frame| match self.view {
                View::Main => self.render(frame),
                View::History => thv.render(frame),
            })?;
        }
    }

    fn handle_input(&mut self) {
        if event::poll(Duration::from_millis(200)).unwrap() {
            let e = event::read().unwrap();
            match e.as_key_event().unwrap().code {
                KeyCode::Char('q') => self.quit = true,
                KeyCode::Char(' ') => {
                    self.timer_widget.toggle();
                    self.seed = Some(random());
                    self.config = Some(TreeConfig {
                        max_x: terminal::size().unwrap().0 / 2,
                        max_y: terminal::size().unwrap().1,
                        life: 30,
                        multiplier: 3,
                    });
                    self.bonsai_widget =
                        BonsaiWidget::new(self.config.unwrap(), self.seed.unwrap());
                }
                KeyCode::Left | KeyCode::Char('h') => {
                    self.timer_widget.move_selector_left();
                }
                KeyCode::Right | KeyCode::Char('l') => {
                    self.timer_widget.move_selector_right();
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    self.timer_widget.modify_goal_time(1);
                }
                KeyCode::Down | KeyCode::Char('j') => self.timer_widget.modify_goal_time(-1),
                KeyCode::Char('n') => self.view = View::History,
                KeyCode::Char('m') => self.view = View::Main,
                _ => (),
            };
        }
    }

    fn render(&self, frame: &mut Frame) {
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
}
