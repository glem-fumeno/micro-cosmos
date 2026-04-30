use std::f32::consts::TAU;

use bevy::prelude::*;
use rand::RngExt;

#[derive(Resource)]
pub struct WindowState {
    pub width: f32,
    pub height: f32,
    pub scale: f32,
    pub sector_size: i32,
    pub cursor: Vec2,
}

impl Default for WindowState {
    fn default() -> Self {
        Self {
            width: 256.,
            height: 144.,
            scale: 1.,
            sector_size: 8,
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

#[derive(Resource, Default)]
pub struct Materials {
    collision: Option<Handle<ColorMaterial>>,
    player: Option<Handle<ColorMaterial>>,
    player_cone: Option<Handle<ColorMaterial>>,
    enemy: Option<Handle<ColorMaterial>>,
    projectile: Option<Handle<ColorMaterial>>,
    background: Option<Handle<ColorMaterial>>,
}

impl Materials {
    pub fn init(&mut self, materials: &mut Assets<ColorMaterial>) {
        self.collision = Some(materials.add(Color::oklch(0.9, 0.01, 30.)));
        self.player = Some(materials.add(Color::oklch(0.75, 0.1, 130.)));
        self.player_cone =
            Some(materials.add(Color::oklcha(0.75, 0.1, 130., 0.1)));
        self.enemy = Some(materials.add(Color::oklch(0.75, 0.1, 30.)));
        self.projectile = Some(materials.add(Color::oklch(0.75, 0.1, 220.)));
        self.background = Some(materials.add(Color::oklch(0.25, 0.01, 30.)));
    }
    pub fn collision(&self) -> Handle<ColorMaterial> {
        self.collision.as_ref().unwrap().clone()
    }
    pub fn player(&self) -> Handle<ColorMaterial> {
        self.player.as_ref().unwrap().clone()
    }
    pub fn player_cone(&self) -> Handle<ColorMaterial> {
        self.player_cone.as_ref().unwrap().clone()
    }
    pub fn enemy(&self) -> Handle<ColorMaterial> {
        self.enemy.as_ref().unwrap().clone()
    }
    pub fn projectile(&self) -> Handle<ColorMaterial> {
        self.projectile.as_ref().unwrap().clone()
    }
    pub fn background(&self) -> Handle<ColorMaterial> {
        self.background.as_ref().unwrap().clone()
    }
}

#[derive(Resource, Default)]
pub struct Meshes {
    projectile: Option<Handle<Mesh>>,
    player: Option<Handle<Mesh>>,
    player_cone: Option<Handle<Mesh>>,
    enemy: Option<Handle<Mesh>>,
}

impl Meshes {
    pub fn init(&mut self, meshes: &mut Assets<Mesh>) {
        self.player = Some(meshes.add(Circle::new(4.0)));
        self.player_cone =
            Some(meshes.add(CircularSector::new(50.0, TAU / 12.)));
        self.enemy = Some(meshes.add(Circle::new(2.0)));
        self.projectile = Some(meshes.add(Circle::new(1.0)));
    }
    pub fn player(&self) -> Handle<Mesh> {
        self.player.as_ref().unwrap().clone()
    }
    pub fn player_cone(&self) -> Handle<Mesh> {
        self.player_cone.as_ref().unwrap().clone()
    }
    pub fn enemy(&self) -> Handle<Mesh> {
        self.enemy.as_ref().unwrap().clone()
    }
    pub fn projectile(&self) -> Handle<Mesh> {
        self.projectile.as_ref().unwrap().clone()
    }
}

#[derive(Resource)]
pub struct Sounds {
    pub click: Handle<AudioSource>,
    pub pop: Handle<AudioSource>,
    pub now_playing: usize,
}

impl FromWorld for Sounds {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        Self {
            click: asset_server.load("click.ogg"),
            pop: asset_server.load("pop.ogg"),
            now_playing: 0,
        }
    }
}
