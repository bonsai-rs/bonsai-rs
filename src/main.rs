use std::time::{Duration, Instant};

use color_eyre::Result;
use crossterm::event::{self, KeyCode};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Flex, Layout},
};

mod widgets;
use widgets::timer_widget::TimerWidget;

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let mut app = App::new();
    let result = app.run(terminal);
    ratatui::restore();
    result
}

struct App {
    timer_widget: TimerWidget,
    quit: bool,
}

impl App {
    const fn new() -> Self {
        App {
            timer_widget: TimerWidget::new(),
            quit: false,
        }
    }

    pub fn run(&mut self, mut terminal: DefaultTerminal) -> Result<()> {
        loop {
            if self.quit {
                return Result::Ok(());
            }
            self.handle_input();
            self.timer_widget.update();
            terminal.draw(|frame: &mut Frame| {
                self.render(frame);
            })?;
        }
    }

    fn handle_input(&mut self) {
        if event::poll(Duration::from_millis(50)).unwrap() {
            let e = event::read().unwrap();
            match e.as_key_event().unwrap().code {
                KeyCode::Char('q') => self.quit = true,
                KeyCode::Char(' ') => self.timer_widget.toggle(),
                KeyCode::Left | KeyCode::Char('h') => {
                    self.timer_widget.move_selector_left();
                }
                KeyCode::Right | KeyCode::Char('l') => {
                    self.timer_widget.move_selector_right();
                }
                KeyCode::Up | KeyCode::Char('j') => {
                    self.timer_widget.modify_goal_time(1);
                }
                KeyCode::Down | KeyCode::Char('k') => self.timer_widget.modify_goal_time(-1),
                _ => (),
            };
        }
    }

    fn render(&self, frame: &mut Frame) {
        let horizontal =
            Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                .flex(Flex::Center);
        let [tree_area, timer_area] = horizontal.areas(frame.area());
        self.timer_widget.render(frame, timer_area);
    }
}
