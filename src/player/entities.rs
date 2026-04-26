use std::f32::consts::TAU;

use bevy::prelude::*;

use crate::{
    components::{Collision, LocalTransform},
    player::components::{Player, PlayerMesh},
};

pub fn spawn_player(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    velocity: f32,
    cooldown: f32,
    attack_cooldown: f32,
) {
    let cone = meshes.add(CircularSector::new(50.0, TAU / 12.));
    let player = meshes.add(Circle::new(5.0));
    let color = Color::hsl(60. as f32 as f32, 0.15, 0.7);
    let opaque_material = materials.add(color);
    let transparent_material = materials.add(color.with_alpha(0.3));
    let mesh_entity = commands
        .spawn((
            PlayerMesh { scale: 1. },
            Mesh2d(cone.clone()),
            MeshMaterial2d(transparent_material.clone()),
            Transform::default(),
        ))
        .id();
    let cooldown_entity = commands
        .spawn((
            PlayerMesh { scale: 1. },
            Mesh2d(cone.clone()),
            MeshMaterial2d(transparent_material.clone()),
            Transform::default(),
        ))
        .id();
    let attack_entity = commands
        .spawn((
            PlayerMesh { scale: 0. },
            Mesh2d(cone.clone()),
            MeshMaterial2d(opaque_material.clone()),
            Transform::default(),
        ))
        .id();
    commands.spawn((
        Player {
            velocity,
            cooldown,
            attack_cooldown,
            current_cooldown: 0.,
            mesh_entity,
            cooldown_entity,
            attack_entity,
        },
        Transform::default(),
        LocalTransform::from_xyz(0., 0., 1.),
        Mesh2d(player.clone()),
        MeshMaterial2d(opaque_material.clone()),
        Collision::new(5.),
    ));
}
