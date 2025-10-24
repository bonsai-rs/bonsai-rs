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

struct App {
    timer_startpoint: Option<Instant>,
    goal_duration: Duration,
    timer_display: String,
}

impl App {
    const fn new() -> Self {
        App {
            timer_startpoint: None,
            goal_duration: Duration::from_secs(60 * 45),
            timer_display: String::new(),
        }
    }

    pub fn run(&mut self, mut terminal: DefaultTerminal) -> Result<()> {
        loop {
            if event::poll(Duration::from_millis(50))? {
                let e = event::read()?;
                match e.as_key_event().unwrap().code {
                    KeyCode::Char(' ') => self.timer_startpoint = Some(Instant::now()),
                    KeyCode::Char('c') => unimplemented!(),
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

    fn render(&self, frame: &mut Frame) {
        let horizontal =
            Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                .flex(Flex::Center);
        let [tree_area, timer_area] = horizontal.areas(frame.area());

        let block = Block::bordered();
        frame.render_widget(block, timer_area);

        let timer = BigText::builder()
            .pixel_size(tui_big_text::PixelSize::Quadrant)
            .lines(vec![self.timer_display.clone().into()])
            .centered()
            .build();

        let [_, _, timer_area, _, _] = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Fill(1),
            Constraint::Fill(1),
            Constraint::Fill(1),
            Constraint::Fill(1),
        ])
        .flex(Flex::Center)
        .areas(timer_area);
        frame.render_widget(timer, timer_area);
    }
}
