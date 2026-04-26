use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub velocity: f32,
    pub cooldown: f32,
    pub current_cooldown: f32,
    pub attack_cooldown: f32,
    pub mesh_entity: Entity,
    pub cooldown_entity: Entity,
    pub attack_entity: Entity,
}

#[derive(Component)]
pub struct PlayerMesh {
    pub scale: f32,
}
