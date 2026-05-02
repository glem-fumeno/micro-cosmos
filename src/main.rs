use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin,
    input_focus::InputFocus,
    prelude::*,
    window::{PresentMode, WindowMode},
};

use crate::{
    app_state::AppState,
    enemies::{entities::spawn_bulk, systems::enemy_timer_spawn},
    menu::{handle_button, handle_start_button, init_menu},
    player::systems::{
        player_attack, player_cooldown, player_despawn, player_move,
        player_rotate, player_transform_mesh,
    },
    resources::{
        Colors, Fonts, Materials, Meshes, Rng, Sounds, Textures, WindowState,
    },
    systems::{
        advance_local_transform, deinit_scene, handle_collision,
        handle_collision_time, handle_edge_collision, handle_fps_count,
        handle_health, handle_play_limit, handle_ttl, handle_window, init_app,
        init_game, local_to_global_transform,
    },
};

pub mod app_state;
pub mod components;
pub mod enemies;
pub mod math;
pub mod menu;
pub mod player;
pub mod projectiles;
pub mod resources;
pub mod systems;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        mode: WindowMode::BorderlessFullscreen(
                            MonitorSelection::Primary,
                        ),
                        present_mode: PresentMode::AutoNoVsync,
                        resizable: true,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
            FrameTimeDiagnosticsPlugin::default(),
        ))
        .init_resource::<InputFocus>()
        .init_resource::<Colors>()
        .init_resource::<WindowState>()
        .init_resource::<Rng>()
        .init_resource::<Meshes>()
        .init_resource::<Materials>()
        .init_resource::<Sounds>()
        .init_resource::<Textures>()
        .init_resource::<Fonts>()
        .init_state::<AppState>()
        .add_systems(Startup, init_app)
        .add_systems(OnEnter(AppState::MainMenu), init_menu)
        .add_systems(OnEnter(AppState::Game), init_game)
        .add_systems(OnExit(AppState::MainMenu), deinit_scene)
        .add_systems(OnExit(AppState::Game), deinit_scene)
        .add_systems(Update, handle_button)
        .add_systems(Update, handle_start_button)
        .add_systems(Update, handle_window)
        .add_systems(Update, advance_local_transform)
        .add_systems(Update, player_move)
        .add_systems(Update, player_rotate)
        .add_systems(Update, player_attack)
        .add_systems(Update, player_cooldown)
        .add_systems(Update, player_despawn.run_if(in_state(AppState::Game)))
        .add_systems(Update, enemy_timer_spawn)
        .add_systems(Update, spawn_bulk)
        .add_systems(Update, handle_collision)
        .add_systems(Update, handle_edge_collision)
        .add_systems(Update, handle_fps_count)
        .add_systems(Update, handle_ttl)
        .add_systems(Update, handle_health)
        .add_systems(Update, handle_collision_time)
        .add_systems(Update, handle_play_limit)
        .add_systems(PostUpdate, local_to_global_transform)
        .add_systems(PostUpdate, player_transform_mesh)
        .run();
}
