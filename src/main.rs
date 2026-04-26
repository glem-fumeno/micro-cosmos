use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::{
    enemies::entities::spawn_enemy,
    player::systems::{
        player_attack, player_cooldown, player_move, player_rotate,
        player_transform_mesh,
    },
    resources::{Rng, WindowState},
    systems::{
        advance_local_transform, handle_collision, handle_edge_collision, handle_window, local_to_global_transform, setup
    },
};

pub mod components;
pub mod enemies;
pub mod math;
pub mod player;
pub mod resources;
pub mod systems;

fn main() {
    App::new()
        .insert_resource(WindowState::default())
        .insert_resource(Rng::default())
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                // mode: WindowMode::BorderlessFullscreen(
                //     MonitorSelection::Primary,
                // ),
                resizable: true,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, handle_window)
        .add_systems(Update, advance_local_transform)
        .add_systems(Update, player_move)
        .add_systems(Update, player_rotate)
        .add_systems(Update, player_attack)
        .add_systems(Update, player_cooldown)
        .add_systems(Update, local_to_global_transform)
        .add_systems(Update, player_transform_mesh)
        .add_systems(
            Update,
            spawn_enemy.run_if(input_just_pressed(KeyCode::Space)),
        )
        // .add_systems(Update, enemy_follow_player)
        .add_systems(Update, handle_collision)
        .add_systems(Update, handle_edge_collision)
        .run();
}
