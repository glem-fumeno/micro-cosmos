use bevy::prelude::*;

#[derive(Component, Clone, Copy)]
pub struct LocalTransform {
    pub angle: f32,
    pub scale: Vec2,
    pub position: Vec2,
    pub velocity: Vec2,
    pub zindex: f32,
    pub acceleration: Vec2,
}
impl LocalTransform {
    pub fn from_xyz(x: f32, y: f32, z: f32) -> Self {
        Self {
            position: vec2(x, y),
            angle: 0.,
            scale: vec2(1., 1.),
            velocity: Vec2::ZERO,
            acceleration: Vec2::ZERO,
            zindex: z,
        }
    }
    pub fn with_velocity(mut self, x: f32, y: f32) -> Self {
        self.velocity.x = x;
        self.velocity.y = y;
        self
    }
}

#[derive(Component, Clone, Copy)]
pub struct Collision {
    pub radius: f32,
}

impl Collision {
    pub fn new(radius: f32) -> Self {
        Self { radius }
    }
}
