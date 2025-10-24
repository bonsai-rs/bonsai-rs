use rand::{SeedableRng, rngs::StdRng};
use ratatui::{
    style::{Color, Style, Styled},
    widgets::{Widget, canvas::Canvas},
};
use rbonsai::bonsai::{self, TreeConfig, Val};

pub struct BonsaiWidget {
    config: TreeConfig,
    tree: Vec<Val>,
    growth: f32,
}

impl BonsaiWidget {
    pub fn new(config: TreeConfig, seed: u64) -> Self {
        let mut rng = StdRng::seed_from_u64(seed);
        Self {
            growth: 0.0,
            tree: bonsai::grow_tree(&config, &mut rng),
            config,
        }
    }

    pub fn get(&self) -> impl Widget + '_ {
        Canvas::default()
            .x_bounds([0f64, self.config.max_x.into()])
            .y_bounds([0f64, self.config.max_y.into()])
            .paint(|ctx| {
                for elm in &self.tree[0..(self.tree.len() as f32 / 100.0 * self.growth) as usize] {
                    ctx.print(
                        elm.pos.x.into(),
                        (self.config.max_y as i32 - elm.pos.y).into(),
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
