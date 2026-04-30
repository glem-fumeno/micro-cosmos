use bevy::prelude::*;

#[derive(Component, Clone, Copy, Debug)]
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
#[derive(Clone, Copy, Debug)]
pub enum CollisionLayer {
    Player,
    Enemy,
    Projectile,
}
impl CollisionLayer {
    pub fn collides_with(&self, other: Self) -> bool {
        match self {
            Self::Player => match other {
                Self::Player => true,
                Self::Enemy => true,
                Self::Projectile => false,
            },
            Self::Enemy => match other {
                Self::Player => true,
                Self::Enemy => true,
                Self::Projectile => true,
            },
            Self::Projectile => match other {
                Self::Projectile => false,
                Self::Player => false,
                Self::Enemy => true,
            },
        }
    }
}

#[derive(Component, Clone, Copy, Debug)]
pub struct Collision {
    pub radius: f32,
    pub layer: CollisionLayer,
    pub mass: f32,
    pub energy: f32,
}

impl Collision {
    pub fn new(radius: f32, layer: CollisionLayer, mass: f32) -> Self {
        Self {
            radius,
            layer,
            mass,
            energy: 0.0,
        }
    }
}

#[derive(Component)]
pub struct CollisionTimer {
    pub material: Option<Handle<ColorMaterial>>,
    pub timer: Timer,
}

impl CollisionTimer {
    pub fn new(duration: f32) -> Self {
        let mut timer = Timer::from_seconds(duration, TimerMode::Once);
        timer.finish();
        Self {
            material: None,
            timer,
        }
    }
}

#[derive(Component)]
pub struct FPSCounter;

#[derive(Component, Deref, DerefMut)]
pub struct TTL(pub Timer);

#[derive(Component)]
pub struct EnergyDisplay;

#[derive(Component)]
pub struct Health {
    pub health: f32,
}
