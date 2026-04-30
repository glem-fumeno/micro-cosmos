use std::f32::consts::TAU;

use bevy::prelude::*;

use crate::{
    components::{Collision, CollisionLayer, CollisionTimer, LocalTransform},
    player::components::{Player, PlayerMesh},
    resources::{Materials, Meshes},
};

pub fn spawn_player(
    commands: &mut Commands,
    meshes: &Meshes,
    materials: &Materials,
    velocity: f32,
    cooldown: f32,
    attack_cooldown: f32,
) {
    let mesh_entity = commands
        .spawn((
            PlayerMesh { scale: 1. },
            Mesh2d(meshes.player_cone()),
            MeshMaterial2d(materials.player_cone()),
            Transform::default(),
        ))
        .id();
    let cooldown_entity = commands
        .spawn((
            PlayerMesh { scale: 1. },
            Mesh2d(meshes.player_cone()),
            MeshMaterial2d(materials.player_cone()),
            Transform::default(),
        ))
        .id();
    let attack_entity = commands
        .spawn((
            PlayerMesh { scale: 0. },
            Mesh2d(meshes.player_cone()),
            MeshMaterial2d(materials.player()),
            Transform::default(),
        ))
        .id();
    commands.spawn((
        Player {
            velocity,
            cooldown,
            attack_cooldown,
            current_cooldown: 0.,
            projectiles: 8,
            spread: TAU / 6.,
            mesh_entity,
            cooldown_entity,
            attack_entity,
        },
        Transform::default(),
        LocalTransform::from_xyz(0., 0., 1.),
        Mesh2d(meshes.player()),
        MeshMaterial2d(materials.player()),
        Collision::new(4., CollisionLayer::Player, 400.),
        CollisionTimer::new(0.1),
    ));
}
