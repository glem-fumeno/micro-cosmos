use std::f32::consts::TAU;

use bevy::prelude::*;

use crate::{
    app_state::AppState,
    components::{Collision, LocalTransform},
    player::components::{Player, PlayerMesh},
    projectiles::entities::spawn_projectiles,
    resources::{Textures, WindowState},
};
pub fn player_move(
    player: Single<&Player, With<LocalTransform>>,
    mut transform: Single<&mut LocalTransform, With<Player>>,
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
    transform.acceleration = velocity_vector * player.velocity;
}

pub fn player_rotate(
    mut transform: Single<&mut LocalTransform, With<Player>>,
    window_state: Res<WindowState>,
) {
    let diff = vec2(
        transform.position.x - window_state.cursor.x,
        transform.position.y - window_state.cursor.y,
    );
    transform.angle = diff.y.atan2(diff.x) + TAU / 4.;
}

pub fn player_attack(
    transform: Single<&mut LocalTransform, With<Player>>,
    mut player: Single<&mut Player, With<LocalTransform>>,
    input: Res<ButtonInput<MouseButton>>,
    mut commands: Commands,
    textures: Res<Textures>,
) {
    if !input.pressed(MouseButton::Left) {
        return;
    }
    if player.current_cooldown > 0. {
        return;
    }
    player.current_cooldown = player.cooldown;
    spawn_projectiles(
        &mut commands,
        &textures,
        &transform,
        player.projectiles,
        player.spread,
    );
}

pub fn player_cooldown(
    mut player: Single<&mut Player>,
    mut query_meshes: Query<&mut PlayerMesh>,
    time: Res<Time>,
) {
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

pub fn player_health(
    player: Single<&mut Player>,
    collision: Single<&Collision, With<Player>>,
    mut query_meshes: Query<&mut PlayerMesh>,
) {
    if let Ok(mut mesh) = query_meshes.get_mut(player.health_entity) {
        mesh.scale =
            ((1. - collision.energy / player.health) * 8.) as i32 as f32 / 8.;
    }
}

pub fn player_transform_mesh(
    player: Single<&Player, With<Transform>>,
    player_transform: Single<&Transform, With<Player>>,
    mut query_mesh: Query<(&mut Transform, &PlayerMesh), Without<Player>>,
) {
    if let Ok((mut transform, mesh)) = query_mesh.get_mut(player.mesh_entity) {
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
    if let Ok((mut transform, mesh)) = query_mesh.get_mut(player.attack_entity)
    {
        *transform = player_transform
            .with_translation(player_transform.translation.with_z(3.))
            .with_scale(player_transform.scale * mesh.scale);
    }
    if let Ok((mut transform, mesh)) = query_mesh.get_mut(player.health_entity)
    {
        *transform = player_transform
            .with_translation(
                player_transform
                    .translation
                    .with_z(player_transform.translation.z + 2.)
                    .with_y(
                        player_transform.translation.y
                            + 6. * player_transform.scale.y,
                    ),
            )
            .with_rotation(Quat::from_rotation_z(0.))
            .with_scale(
                player_transform
                    .scale
                    .with_x(player_transform.scale.x * mesh.scale),
            );
    }
    if let Ok((mut transform, mesh)) =
        query_mesh.get_mut(player.health_background_entity)
    {
        *transform = player_transform
            .with_translation(
                player_transform
                    .translation
                    .with_z(player_transform.translation.z + 1.)
                    .with_y(
                        player_transform.translation.y
                            + 6. * player_transform.scale.y,
                    ),
            )
            .with_rotation(Quat::from_rotation_z(0.))
            .with_scale(player_transform.scale * mesh.scale);
    }
}

pub fn player_despawn(
    query: Query<&Player>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if query.iter().len() < 1 {
        next_state.set(AppState::MainMenu);
    }
}
