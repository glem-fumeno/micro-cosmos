use std::f32::consts::TAU;

use bevy::prelude::*;

use crate::{
    components::LocalTransform,
    player::components::{Player, PlayerMesh},
    projectiles::entities::spawn_projectiles,
    resources::{Materials, Meshes, WindowState},
};
pub fn player_move(
    mut query: Query<(&Player, &mut LocalTransform)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    let mut velocity_vector = Vec2::ZERO;
    if input.pressed(KeyCode::KeyW) {
        velocity_vector.y += 1.;
    }
    if input.pressed(KeyCode::KeyS) {
        velocity_vector.y -= 1.;
    }
    if input.pressed(KeyCode::KeyA) {
        velocity_vector.x -= 1.;
    }
    if input.pressed(KeyCode::KeyD) {
        velocity_vector.x += 1.;
    }
    for (player, mut transform) in &mut query {
        transform.velocity = velocity_vector * player.velocity;
    }
}

pub fn player_rotate(
    mut query: Query<&mut LocalTransform, With<Player>>,
    window_state: Res<WindowState>,
) {
    for mut transform in &mut query {
        let diff = vec2(
            transform.position.x - window_state.cursor.x,
            transform.position.y - window_state.cursor.y,
        );
        transform.angle = diff.y.atan2(diff.x) + TAU / 4.;
    }
}

pub fn player_attack(
    mut query: Query<(&LocalTransform, &mut Player)>,
    input: Res<ButtonInput<MouseButton>>,
    mut commands: Commands,
    meshes: Res<Meshes>,
    materials: Res<Materials>,
) {
    if !input.pressed(MouseButton::Left) {
        return;
    }
    for (transform, mut player) in &mut query {
        if player.current_cooldown <= 0. {
            player.current_cooldown = player.cooldown;
            spawn_projectiles(
                &mut commands,
                &meshes,
                &materials,
                *transform,
                player.projectiles,
                player.spread,
            );
        }
    }
}

pub fn player_cooldown(
    mut query_player: Query<&mut Player>,
    mut query_meshes: Query<&mut PlayerMesh>,
    time: Res<Time>,
) {
    for mut player in &mut query_player {
        player.current_cooldown -= time.delta_secs();
        if player.current_cooldown < 0. {
            player.current_cooldown = 0.;
        }
        if let Ok(mut mesh) = query_meshes.get_mut(player.cooldown_entity) {
            mesh.scale = 1. - player.current_cooldown / player.cooldown;
        }
        if let Ok(mut mesh) = query_meshes.get_mut(player.attack_entity) {
            if player.current_cooldown > player.attack_cooldown {
                mesh.scale = 1.
            } else {
                mesh.scale = 0.
            }
        }
    }
}

pub fn player_transform_mesh(
    query_player: Query<(&Transform, &Player)>,
    mut query_mesh: Query<(&mut Transform, &PlayerMesh), Without<Player>>,
) {
    for (player_transform, player) in &query_player {
        if let Ok((mut transform, mesh)) =
            query_mesh.get_mut(player.mesh_entity)
        {
            *transform = player_transform
                .with_translation(player_transform.translation.with_z(1.))
                .with_scale(player_transform.scale * mesh.scale);
        }
        if let Ok((mut transform, mesh)) =
            query_mesh.get_mut(player.cooldown_entity)
        {
            *transform = player_transform
                .with_translation(player_transform.translation.with_z(2.))
                .with_scale(player_transform.scale * mesh.scale);
        }
        if let Ok((mut transform, mesh)) =
            query_mesh.get_mut(player.attack_entity)
        {
            *transform = player_transform
                .with_translation(player_transform.translation.with_z(3.))
                .with_scale(player_transform.scale * mesh.scale);
        }
    }
}
