use bevy::prelude::*;
use rand::RngExt;

#[derive(Resource)]
pub struct WindowState {
    pub width: f32,
    pub height: f32,
    pub scale: f32,
    pub cursor: Vec2,
}

impl Default for WindowState {
    fn default() -> Self {
        Self {
            width: 256.,
            height: 144.,
            scale: 1.,
            cursor: Vec2::ZERO,
        }
    }
}

#[derive(Resource)]
pub struct Rng {
    pub rng: rand::rngs::StdRng,
}

impl Default for Rng {
    fn default() -> Self {
        Self {
            rng: rand::make_rng(),
        }
    }
}
impl Rng {
    pub fn random_to(&mut self, value: f32) -> f32 {
        self.rng.random_range((0.)..value)
    }
}
