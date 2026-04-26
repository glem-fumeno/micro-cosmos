use bevy::prelude::*;

use crate::{
    components::LocalTransform, enemies::components::Enemy,
    player::components::Player,
};

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
