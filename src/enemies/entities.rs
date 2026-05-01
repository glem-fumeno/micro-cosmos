use std::f32::consts::TAU;

use bevy::prelude::*;

use crate::{
    components::{
        Collision, CollisionLayer, CollisionTimer, Health, LocalTransform,
    },
    enemies::components::Enemy,
    resources::{Rng, Textures, WindowState},
};
pub fn spawn_bulk(
    mut commands: Commands,
    textures: Res<Textures>,
    window: Res<WindowState>,
    mut rng: ResMut<Rng>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if !input.just_pressed(KeyCode::Space) {
        return;
    }
    for _ in 0..1000 {
        spawn_enemy(&mut commands, &window, &textures, &mut rng);
    }
}

pub fn spawn_enemy(
    commands: &mut Commands,
    window: &WindowState,
    textures: &Textures,
    rng: &mut Rng,
) {
    let index = rng.randomi_to(9) + 1;
    commands.spawn((
        Enemy {
            velocity: 40.,
            index,
        },
        LocalTransform::from_xyz(
            rng.random_to(window.width) - window.width / 2.,
            rng.random_to(window.height) - window.height / 2.,
            2.,
        )
        .with_angle(rng.random_to(TAU))
        .with_velocity(rng.random_to(20.) - 10., rng.random_to(20.) - 10.),
        Transform::default(),
        Collision::new(2., CollisionLayer::Enemy, 100.),
        CollisionTimer::new(0.1),
        Health { health: 5. },
        Sprite::from_image(textures.enemy_from_index(index).unwrap()),
    ));
}
