use bevy::prelude::*;

use crate::{
    components::{Collision, LocalTransform},
    enemies::components::Enemy,
    resources::{Rng, WindowState},
};

pub fn spawn_enemy(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    window: &WindowState,
    rng: &mut Rng,
) {
    let mesh = meshes.add(Circle::new(2.0));
    let color = Color::hsl(0. as f32 as f32, 0.75, 0.7);
    let material = materials.add(color);
    commands.spawn((
        Enemy { velocity: 40. },
        Mesh2d(mesh.clone()),
        MeshMaterial2d(material.clone()),
        LocalTransform::from_xyz(
            rng.random_to(window.width) - window.width / 2.,
            rng.random_to(window.height) - window.height / 2.,
            2.,
        )
        .with_velocity(rng.random_to(20.) - 10., rng.random_to(20.) - 10.),
        Transform::default(),
        Collision::new(2.),
    ));
}
