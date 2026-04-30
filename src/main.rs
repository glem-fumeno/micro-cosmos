use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin,
    prelude::*,
    window::{PresentMode, WindowMode},
};

use crate::{
    enemies::systems::enemy_timer_spawn,
    player::systems::{
        player_attack, player_cooldown, player_move, player_rotate,
        player_transform_mesh,
    },
    resources::{Materials, Meshes, Rng, Sounds, WindowState},
    systems::{
        advance_local_transform, handle_collision, handle_collision_time,
        handle_edge_collision, handle_energy, handle_fps_count, handle_health,
        handle_play_limit, handle_ttl, handle_window,
        local_to_global_transform, setup,
    },
};

pub mod components;
pub mod enemies;
pub mod math;
pub mod player;
pub mod projectiles;
pub mod resources;
pub mod systems;

fn main() {
    App::new()
        .insert_resource(WindowState::default())
        .insert_resource(Rng::default())
        .insert_resource(Materials::default())
        .insert_resource(Meshes::default())
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    mode: WindowMode::BorderlessFullscreen(
                        MonitorSelection::Primary,
                    ),
                    present_mode: PresentMode::AutoNoVsync,
                    resizable: true,
                    ..default()
                }),
                ..default()
            }),
            FrameTimeDiagnosticsPlugin::default(),
        ))
        .init_resource::<Sounds>()
        .add_systems(Startup, setup)
        .add_systems(Update, handle_window)
        .add_systems(Update, advance_local_transform)
        .add_systems(Update, player_move)
        .add_systems(Update, player_rotate)
        .add_systems(Update, player_attack)
        .add_systems(Update, player_cooldown)
        .add_systems(Update, enemy_timer_spawn)
        .add_systems(Update, handle_collision)
        .add_systems(Update, handle_edge_collision)
        .add_systems(Update, handle_fps_count)
        .add_systems(Update, handle_ttl)
        .add_systems(Update, handle_energy)
        .add_systems(Update, handle_health)
        .add_systems(Update, handle_collision_time)
        .add_systems(Update, handle_play_limit)
        .add_systems(PostUpdate, local_to_global_transform)
        .add_systems(PostUpdate, player_transform_mesh)
        .run();
}
