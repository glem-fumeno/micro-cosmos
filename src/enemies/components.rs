use bevy::prelude::*;
#[derive(Component)]
pub struct Enemy {
    pub velocity: f32,
    pub index: i32,
}

#[derive(Component, Deref, DerefMut)]
pub struct SpawnTimer(pub Timer);
