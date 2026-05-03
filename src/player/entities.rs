use std::f32::consts::TAU;

use bevy::prelude::*;

use crate::{
    components::{
        Collision, CollisionLayer, CollisionTimer, Health, LocalTransform,
        SceneEntity,
    },
    player::components::{Player, PlayerMesh},
    resources::{Materials, Meshes, Textures},
};

pub fn spawn_player(
    commands: &mut Commands,
    meshes: &Meshes,
    materials: &Materials,
    textures: &Textures,
    velocity: f32,
    cooldown: f32,
    attack_cooldown: f32,
    health: f32,
) {
    let mesh_entity = commands
        .spawn((
            SceneEntity,
            PlayerMesh { scale: 1. },
            Mesh2d(meshes.player_cone.clone()),
            MeshMaterial2d(materials.player_cone.clone()),
            Transform::default(),
        ))
        .id();
    let cooldown_entity = commands
        .spawn((
            SceneEntity,
            PlayerMesh { scale: 1. },
            Mesh2d(meshes.player_cone.clone()),
            MeshMaterial2d(materials.player_cone.clone()),
            Transform::default(),
        ))
        .id();
    let attack_entity = commands
        .spawn((
            SceneEntity,
            PlayerMesh { scale: 0. },
            Mesh2d(meshes.player_cone.clone()),
            MeshMaterial2d(materials.player.clone()),
            Transform::default(),
        ))
        .id();
    let health_entity = commands
        .spawn((
            SceneEntity,
            PlayerMesh { scale: 1. },
            Mesh2d(meshes.health.clone()),
            MeshMaterial2d(materials.player.clone()),
            Transform::default(),
        ))
        .id();
    let health_background_entity = commands
        .spawn((
            SceneEntity,
            PlayerMesh { scale: 1. },
            Mesh2d(meshes.health.clone()),
            MeshMaterial2d(materials.health.clone()),
            Transform::default(),
        ))
        .id();
    commands.spawn((
        SceneEntity,
        Player {
            velocity,
            cooldown,
            attack_cooldown,
            current_cooldown: 0.,
            projectiles: 8,
            spread: TAU / 6.,
            health,
            mesh_entity,
            cooldown_entity,
            attack_entity,
            health_entity,
            health_background_entity,
        },
        Transform::default(),
        LocalTransform::from_xyz(0., 0., 1.),
        Sprite::from_image(textures.player.clone()),
        Collision::new(4., CollisionLayer::Player, 400.),
        Health { health },
        CollisionTimer::new(0.1),
    ));
}
