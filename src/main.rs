use std::time::Duration;

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

        let block = Block::bordered();
        frame.render_widget(block, timer_area);

        let timer = BigText::builder()
            .pixel_size(tui_big_text::PixelSize::Quadrant)
            .lines(vec!["00h 40min 30s".into()])
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
