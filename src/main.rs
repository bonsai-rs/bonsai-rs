use std::{sync::Mutex, thread, time::Duration};

use color_eyre::Result;
use crossterm::event::{self, KeyCode, KeyEvent};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Flex, Layout, Rect},
    widgets::{Block, Padding, Paragraph},
};

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app = App::new();
    let result = app.run(terminal);
    ratatui::restore();
    result
}

struct App {}

impl App {
    const fn new() -> Self {
        App {}
    }

    pub fn run(self, mut terminal: DefaultTerminal) -> Result<()> {
        loop {
            if event::poll(Duration::from_millis(50))? {
                let e = event::read()?;
                match e.as_key_event().unwrap().code {
                    KeyCode::Char(' ') => unimplemented!(),
                    KeyCode::Char('c') => unimplemented!(),
                    _ => (),
                };
            }

            terminal.draw(Self::render)?;
        }
    }

    fn render(frame: &mut Frame) {
        let horizontal =
            Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                .flex(Flex::Center);
        let [tree_area, timer_area] = horizontal.areas(frame.area());

        let timer = Paragraph::new("Timer")
            .block(
                Block::bordered()
                    .padding(Padding::new(0, 0, timer_area.height / 2, 0))
                    .title("Timer"),
            )
            .centered();

        frame.render_widget(timer, timer_area);
    }
}
