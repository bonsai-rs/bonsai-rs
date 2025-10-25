use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Modifier, Style},
    widgets::{Block, Padding, Paragraph},
};

pub struct NetworkingWidget {
    session_token: Option<u16>,
    users: Vec<String>,
}

impl NetworkingWidget {
    pub fn new() -> Self {
        Self {
            session_token: None,
            users: vec![],
        }
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        match self.session_token {
            Some(token) => {
                let block = Block::bordered()
                    .title_top(format!("   {}   ", token))
                    .title_style(Style::default().add_modifier(Modifier::ITALIC))
                    .title_alignment(Alignment::Center);
                frame.render_widget(block, area);
            }
            None => {
                let block = Block::bordered()
                    .title_top("   NO SESSION   ")
                    .title_style(Style::default().add_modifier(Modifier::ITALIC))
                    .title_alignment(Alignment::Center);
                let instructions = Paragraph::new(
                    "[SPACE] to toggle timer\n[G] to generate session token\n[J] to join a session\n[q]uit",
                )
                .centered();

                let vertical = Layout::vertical([
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                ])
                .flex(ratatui::layout::Flex::Center);
                let [_, centered_area, _] = vertical.areas(area);

                frame.render_widget(block, area);
                frame.render_widget(instructions, centered_area);
            }
        }
    }
}
