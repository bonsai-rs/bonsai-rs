use rand::{SeedableRng, rngs::StdRng};
use ratatui::{
    style::{Color, Style, Styled},
    widgets::canvas::{Canvas, Context},
};
use rbonsai::bonsai::{self, TreeConfig, Val};

use crate::util::get_longest_line_len;

pub struct BonsaiWidget {
    config: TreeConfig,
    tree: Vec<Val>,
    growth: f32,
}

#[allow(dead_code)]
impl BonsaiWidget {
    pub fn new(config: TreeConfig, seed: u64) -> Self {
        let mut rng = StdRng::seed_from_u64(seed);
        Self {
            growth: 0.0,
            tree: bonsai::grow_tree(&config, &mut rng),
            config,
        }
    }

    pub fn get(&self) -> Canvas<'_, impl Fn(&mut Context)> {
        Canvas::default()
            .x_bounds([0f64, self.config.max_x.into()])
            .y_bounds([0f64, self.config.max_y.into()])
            .paint(|ctx| {
                let mut base_lines = rbonsai::base::get_base();
                base_lines.reverse();
                let longest_base_line_len = get_longest_line_len(&base_lines);
                for (index, ln) in base_lines.iter().enumerate() {
                    ctx.print(
                        ((self.config.max_x / 2) - (longest_base_line_len / 2) as u16) as f64,
                        index as f64 + 1f64,
                        ln.to_string(),
                    );
                }

                for elm in &self.tree[0..(self.tree.len() as f32 / 100.0 * self.growth) as usize] {
                    ctx.print(
                        elm.pos.x.into(),
                        (self.config.max_y as i32 - elm.pos.y) as f64 + base_lines.len() as f64,
                        elm.char.clone().set_style(
                            Style::default().fg(Color::Indexed(elm.style.foreground_color)),
                        ),
                    );
                }
            })
    }

    pub fn set_growth(&mut self, growth: f32) {
        self.growth = growth;
    }

    pub fn get_growth(&self) -> f32 {
        self.growth
    }

    pub fn increase_growth(&mut self, add_growth: f32) {
        if self.growth > 100.0 {
            return;
        }
        self.growth += add_growth;
    }
}
