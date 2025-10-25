use std::time::{Duration, Instant};

use ratatui::{
    Frame,
    layout::{Constraint, Flex, Layout, Rect},
    widgets::Block,
};
use tui_big_text::BigText;

enum TimeSelection {
    Seconds,
    Minutes,
    Hours,
}

pub struct TimerWidget {
    pub timer_startpoint: Option<Instant>,
    pub goal_duration: Duration,
    timer_display: String,
    time_selector: TimeSelection,
}

impl TimerWidget {
    pub const fn new() -> Self {
        TimerWidget {
            timer_startpoint: None,
            goal_duration: Duration::from_secs(20),
            timer_display: String::new(),
            time_selector: TimeSelection::Hours,
        }
    }

    pub fn toggle(&mut self) {
        if self.timer_startpoint.is_some() {
            self.timer_startpoint = None;
        } else {
            self.timer_startpoint = Some(Instant::now());
        }
    }

    pub fn modify_goal_time(&mut self, delta: i32) {
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

    pub fn move_selector_left(&mut self) {
        self.time_selector = match self.time_selector {
            TimeSelection::Hours => TimeSelection::Seconds,
            TimeSelection::Minutes => TimeSelection::Hours,
            TimeSelection::Seconds => TimeSelection::Minutes,
        }
    }

    pub fn move_selector_right(&mut self) {
        self.time_selector = match self.time_selector {
            TimeSelection::Hours => TimeSelection::Minutes,
            TimeSelection::Minutes => TimeSelection::Seconds,
            TimeSelection::Seconds => TimeSelection::Hours,
        }
    }

    pub fn update(&mut self) {
        let elapsed = match self.timer_startpoint {
            Some(t) => t.elapsed(),
            None => Duration::from_secs(0),
        };

        if elapsed >= self.goal_duration {
            return;
        }

        let display_duration = self.goal_duration - elapsed;
        let seconds = display_duration.as_secs() % 60;
        let minutes = (display_duration.as_secs() / 60) % 60;
        let hours = (display_duration.as_secs() / 60) / 60;

        self.timer_display = format!("{:0>2}h {:0>2}min {:0>2}s", hours, minutes, seconds);
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let block = Block::bordered();
        frame.render_widget(block, area);

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
                "".into(),
                upper_selection_row.into(),
                self.timer_display.clone().into(),
                bottom_selection_row.into(),
                "".into(),
            ])
            .centered()
            .build();

        let [_, timer_area, _] = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Fill(3),
            Constraint::Fill(1),
        ])
        .flex(Flex::Center)
        .areas(area);

        frame.render_widget(timer, timer_area);
    }
}
