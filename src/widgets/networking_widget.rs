use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout, Margin, Rect},
    style::{Modifier, Style},
    widgets::{Block, Paragraph},
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

    pub fn generate_session(&mut self) {
        let token = ureq::get("http://127.0.0.1:8000/session/create/")
            .header("Content-Type", "text/plain")
            .call()
            .unwrap()
            .body_mut()
            .read_to_string()
            .unwrap();
        self.session_token = Some(token.parse().unwrap());
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        match self.session_token {
            Some(token) => {
                let block = Block::bordered()
                    .title_top(format!("   {}   ", token))
                    .title_style(Style::default().add_modifier(Modifier::ITALIC))
                    .title_alignment(Alignment::Center);
                frame.render_widget(block, area);

                let inner_area = area.inner(Margin::new(2, 2));
                let vertical = Layout::vertical([Constraint::Percentage(25), Constraint::Fill(1)]);
                let [_, center_area] = vertical.areas(inner_area);

                // user list
                let mut user_list = String::new();
                for i in 0..self.users.len() {
                    user_list.push_str(format!("{}\n", self.users[i]).as_str());
                }

                let t = Paragraph::new(user_list).centered();
                frame.render_widget(t, center_area);
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
