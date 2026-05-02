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
        self.rng.random_range(0.0..=value)
    }
    pub fn randomi_to(&mut self, value: i32) -> i32 {
        self.rng.random_range(0..=value)
    }
}
#[derive(Resource, Clone, Copy)]
pub struct Colors {
    pub red: Color,
    pub green: Color,
    pub blue: Color,
    pub background: Color,
    pub surface: Color,
    pub hover: Color,
    pub click: Color,
    pub foreground: Color,
}
impl Default for Colors {
    fn default() -> Self {
        return Self {
            red: Color::oklch(0.75, 0.1, 30.),
            green: Color::oklch(0.75, 0.1, 130.),
            blue: Color::oklch(0.75, 0.1, 220.),
            background: Color::oklch(0.25, 0.01, 30.),
            surface: Color::oklch(0.35, 0.01, 30.),
            hover: Color::oklch(0.45, 0.01, 30.),
            click: Color::oklch(0.55, 0.01, 30.),
            foreground: Color::oklch(0.9, 0.01, 30.),
        };
    }
}

#[derive(Resource)]
pub struct Fonts {
    pub lexend: Handle<Font>,
}
impl FromWorld for Fonts {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        Self {
            lexend: asset_server.load("lexend.ttf").into(),
        }
    }
}

#[derive(Resource)]
pub struct Materials {
    pub collision: Handle<ColorMaterial>,
    pub player: Handle<ColorMaterial>,
    pub player_cone: Handle<ColorMaterial>,
    pub enemy: Handle<ColorMaterial>,
    pub projectile: Handle<ColorMaterial>,
    pub background: Handle<ColorMaterial>,
}
impl FromWorld for Materials {
    fn from_world(world: &mut World) -> Self {
        let colors = *world.resource::<Colors>();
        let mut materials = world.resource_mut::<Assets<ColorMaterial>>();
        Self {
            collision: materials.add(colors.foreground),
            player: materials.add(colors.green),
            player_cone: materials.add(colors.green.with_alpha(0.1)),
            enemy: materials.add(colors.red),
            projectile: materials.add(colors.blue),
            background: materials.add(colors.background),
        }
    }
}

#[derive(Resource)]
pub struct Meshes {
    pub projectile: Handle<Mesh>,
    pub player: Handle<Mesh>,
    pub player_cone: Handle<Mesh>,
    pub enemy: Handle<Mesh>,
}
impl FromWorld for Meshes {
    fn from_world(world: &mut World) -> Self {
        let mut meshes = world.resource_mut::<Assets<Mesh>>();
        Self {
            player: meshes.add(Circle::new(4.0)),
            player_cone: meshes.add(CircularSector::new(50.0, TAU / 12.)),
            enemy: meshes.add(Circle::new(2.0)),
            projectile: meshes.add(Circle::new(1.0)),
        }
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

#[derive(Resource)]
pub struct Textures {
    pub enemy_01: Handle<Image>,
    pub enemy_02: Handle<Image>,
    pub enemy_03: Handle<Image>,
    pub enemy_04: Handle<Image>,
    pub enemy_05: Handle<Image>,
    pub enemy_06: Handle<Image>,
    pub enemy_07: Handle<Image>,
    pub enemy_08: Handle<Image>,
    pub enemy_09: Handle<Image>,
    pub enemy_10: Handle<Image>,
    pub enemy_01d: Handle<Image>,
    pub enemy_02d: Handle<Image>,
    pub enemy_03d: Handle<Image>,
    pub enemy_04d: Handle<Image>,
    pub enemy_05d: Handle<Image>,
    pub enemy_06d: Handle<Image>,
    pub enemy_07d: Handle<Image>,
    pub enemy_08d: Handle<Image>,
    pub enemy_09d: Handle<Image>,
    pub enemy_10d: Handle<Image>,
    pub player: Handle<Image>,
    pub projectile: Handle<Image>,
    pub particle: Handle<Image>,
}
impl Textures {
    pub fn enemy_from_index(&self, index: i32) -> Option<Handle<Image>> {
        Some(match index {
            1 => self.enemy_01.clone(),
            2 => self.enemy_02.clone(),
            3 => self.enemy_03.clone(),
            4 => self.enemy_04.clone(),
            5 => self.enemy_05.clone(),
            6 => self.enemy_06.clone(),
            7 => self.enemy_07.clone(),
            8 => self.enemy_08.clone(),
            9 => self.enemy_09.clone(),
            10 => self.enemy_10.clone(),
            _ => None?,
        })
    }
    pub fn enemyd_from_index(&self, index: i32) -> Option<Handle<Image>> {
        Some(match index {
            1 => self.enemy_01d.clone(),
            2 => self.enemy_02d.clone(),
            3 => self.enemy_03d.clone(),
            4 => self.enemy_04d.clone(),
            5 => self.enemy_05d.clone(),
            6 => self.enemy_06d.clone(),
            7 => self.enemy_07d.clone(),
            8 => self.enemy_08d.clone(),
            9 => self.enemy_09d.clone(),
            10 => self.enemy_10d.clone(),
            _ => None?,
        })
    }
}

impl FromWorld for Textures {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        Self {
            enemy_01: asset_server.load("enemies/01.png"),
            enemy_02: asset_server.load("enemies/02.png"),
            enemy_03: asset_server.load("enemies/03.png"),
            enemy_04: asset_server.load("enemies/04.png"),
            enemy_05: asset_server.load("enemies/05.png"),
            enemy_06: asset_server.load("enemies/06.png"),
            enemy_07: asset_server.load("enemies/07.png"),
            enemy_08: asset_server.load("enemies/08.png"),
            enemy_09: asset_server.load("enemies/09.png"),
            enemy_10: asset_server.load("enemies/10.png"),
            enemy_01d: asset_server.load("enemies/d01.png"),
            enemy_02d: asset_server.load("enemies/d02.png"),
            enemy_03d: asset_server.load("enemies/d03.png"),
            enemy_04d: asset_server.load("enemies/d04.png"),
            enemy_05d: asset_server.load("enemies/d05.png"),
            enemy_06d: asset_server.load("enemies/d06.png"),
            enemy_07d: asset_server.load("enemies/d07.png"),
            enemy_08d: asset_server.load("enemies/d08.png"),
            enemy_09d: asset_server.load("enemies/d09.png"),
            enemy_10d: asset_server.load("enemies/d10.png"),
            player: asset_server.load("player.png"),
            projectile: asset_server.load("projectile.png"),
            particle: asset_server.load("particle.png"),
        }
    }
}
