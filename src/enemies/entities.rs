use bevy::prelude::*;

use crate::{
    components::{
        Collision, CollisionLayer, CollisionTimer, Health, LocalTransform,
    },
    enemies::components::Enemy,
    resources::{Materials, Meshes, Rng, WindowState},
};
pub fn spawn_bulk(
    mut commands: Commands,
    meshes: Res<Meshes>,
    materials: Res<Materials>,
    window: Res<WindowState>,
    mut rng: ResMut<Rng>,
) {
    for _ in 0..1000 {
        commands.spawn((
            Enemy { velocity: 40. },
            Mesh2d(meshes.enemy()),
            MeshMaterial2d(materials.enemy()),
            LocalTransform::from_xyz(
                rng.random_to(window.width) - window.width / 2.,
                rng.random_to(window.height) - window.height / 2.,
                2.,
            )
            .with_velocity(rng.random_to(20.) - 10., rng.random_to(20.) - 10.),
            Transform::default(),
            Collision::new(2., CollisionLayer::Enemy, 100.),
            CollisionTimer::new(0.1),
            Health { health: 5. },
        ));
    }
}

pub fn spawn_enemy(
    commands: &mut Commands,
    meshes: &Meshes,
    materials: &Materials,
    window: &WindowState,
    rng: &mut Rng,
) {
    commands.spawn((
        Enemy { velocity: 40. },
        Mesh2d(meshes.enemy()),
        MeshMaterial2d(materials.enemy()),
        LocalTransform::from_xyz(
            rng.random_to(window.width) - window.width / 2.,
            rng.random_to(window.height) - window.height / 2.,
            2.,
        )
        .with_velocity(rng.random_to(20.) - 10., rng.random_to(20.) - 10.),
        Transform::default(),
        Collision::new(2., CollisionLayer::Enemy, 100.),
        CollisionTimer::new(0.1),
        Health { health: 5. },
    ));
}
