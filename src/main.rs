use std::time::{Duration, Instant};

use color_eyre::Result;
use crossterm::event::{self, KeyCode};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Flex, Layout},
    widgets::Block,
};
use tui_big_text::BigText;

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let mut app = App::new();
    let result = app.run(terminal);
    ratatui::restore();
    result
}

enum TimeSelection {
    Hours,
    Minutes,
    Seconds,
}

struct App {
    timer_startpoint: Option<Instant>,
    goal_duration: Duration,
    timer_display: String,
    time_selector: TimeSelection,
}

impl App {
    const fn new() -> Self {
        App {
            timer_startpoint: None,
            goal_duration: Duration::from_secs(60 * 45),
            timer_display: String::new(),
            time_selector: TimeSelection::Minutes,
        }
    }

    pub fn run(&mut self, mut terminal: DefaultTerminal) -> Result<()> {
        loop {
            if event::poll(Duration::from_millis(50))? {
                let e = event::read()?;
                match e.as_key_event().unwrap().code {
                    KeyCode::Char(' ') => self.timer_startpoint = Some(Instant::now()),
                    KeyCode::Char('c') => unimplemented!(),
                    KeyCode::Left | KeyCode::Char('h') => {
                        self.time_selector = match self.time_selector {
                            TimeSelection::Hours => TimeSelection::Seconds,
                            TimeSelection::Minutes => TimeSelection::Hours,
                            TimeSelection::Seconds => TimeSelection::Minutes,
                        }
                    }
                    KeyCode::Right | KeyCode::Char('l') => {
                        self.time_selector = match self.time_selector {
                            TimeSelection::Hours => TimeSelection::Minutes,
                            TimeSelection::Minutes => TimeSelection::Seconds,
                            TimeSelection::Seconds => TimeSelection::Hours,
                        }
                    }
                    KeyCode::Up | KeyCode::Char('j') => {
                        self.modify_goal_time(1);
                    }
                    KeyCode::Down | KeyCode::Char('k') => self.modify_goal_time(-1),
                    _ => (),
                };
            }

            let elapsed = match self.timer_startpoint {
                Some(t) => t.elapsed(),
                None => Duration::from_secs(0),
            };

            let display_duration = self.goal_duration - elapsed;
            let seconds = display_duration.as_secs() % 60;
            let minutes = (display_duration.as_secs() / 60) % 60;
            let hours = (display_duration.as_secs() / 60) / 60;

            self.timer_display = format!("{:0>2}h {:0>2}min {:0>2}s", hours, minutes, seconds);
            terminal.draw(|frame: &mut Frame| {
                self.render(frame);
            })?;
        }
    }

    fn modify_goal_time(&mut self, delta: i32) {
        if self.timer_startpoint.is_some() {
            return;
        }

        let new_seconds = match self.time_selector {
            TimeSelection::Seconds => self
                .goal_duration
                .as_secs()
                .checked_add_signed(delta as i64),
            TimeSelection::Minutes => self
                .goal_duration
                .as_secs()
                .checked_add_signed(delta as i64 * 60),
            TimeSelection::Hours => self
                .goal_duration
                .as_secs()
                .checked_add_signed(delta as i64 * 60 * 60),
        };

        self.goal_duration =
            Duration::from_secs(new_seconds.unwrap_or(self.goal_duration.as_secs()));
    }

    fn render(&self, frame: &mut Frame) {
        let horizontal =
            Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                .flex(Flex::Center);
        let [tree_area, timer_area] = horizontal.areas(frame.area());

        let block = Block::bordered();
        frame.render_widget(block, timer_area);

        let (upper_selection_row, bottom_selection_row) = if self.timer_startpoint.is_some() {
            ("", "")
        } else {
            match self.time_selector {
                TimeSelection::Hours => ("^          ", "v          "),
                TimeSelection::Minutes => ("^", "v"),
                TimeSelection::Seconds => ("          ^", "          v"),
            }
        };

        let timer = BigText::builder()
            .pixel_size(tui_big_text::PixelSize::Quadrant)
            .lines(vec![
                upper_selection_row.into(),
                self.timer_display.clone().into(),
                bottom_selection_row.into(),
            ])
            .centered()
            .build();

        let [_, timer_area, _] = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Fill(1),
            Constraint::Fill(1),
        ])
        .flex(Flex::Center)
        .areas(timer_area);
        frame.render_widget(timer, timer_area);
    }
}
