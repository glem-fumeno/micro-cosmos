use std::time::Duration;

use bevy::prelude::*;

use crate::{
    components::LocalTransform,
    enemies::{
        components::{Enemy, SpawnTimer},
        entities::spawn_enemy,
    },
    player::components::Player,
    resources::{Rng, Textures, WindowState},
};
pub fn enemy_timer_spawn(
    mut timer: Single<&mut SpawnTimer>,
    time: Res<Time>,
    mut commands: Commands,
    textures: Res<Textures>,
    window: Res<WindowState>,
    mut rng: ResMut<Rng>,
) {
    if timer.tick(time.delta()).just_finished() {
        let last_duration = timer.0.duration().as_secs_f64();
        timer.set_duration(Duration::from_secs_f64(last_duration * 0.99));
        timer.reset();
        spawn_enemy(&mut commands, &window, &textures, &mut rng);
    }
}

pub fn enemy_follow_player(
    query_player: Query<&LocalTransform, With<Player>>,
    mut query_enemy: Query<(&mut LocalTransform, &Enemy), Without<Player>>,
) {
    for player_transform in query_player {
        for (mut enemy_transform, enemy) in &mut query_enemy {
            enemy_transform.velocity = (player_transform.position
                - enemy_transform.position)
                .normalize()
                * enemy.velocity;
        }
    }
}
