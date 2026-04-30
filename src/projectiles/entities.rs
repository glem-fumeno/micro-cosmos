use std::f32::consts::PI;

use bevy::prelude::*;

use crate::{
    components::{
        Collision, CollisionLayer, CollisionTimer, LocalTransform, TTL,
    },
    projectiles::components::Projectile,
    resources::{Materials, Meshes},
};

pub fn spawn_projectiles(
    commands: &mut Commands,
    meshes: &Meshes,
    materials: &Materials,
    transform: LocalTransform,
    count: i32,
    spread: f32,
) {
    for i in 0..count {
        let delta = (spread / (count as f32 - 1.)) * (i as f32);
        let angle = transform.angle - spread / 2. + delta;
        let v = Vec2::from_angle(angle + PI / 2.) * 100.;
        commands.spawn((
            Projectile { velocity: 100. },
            TTL(Timer::from_seconds(0.5, TimerMode::Once)),
            Mesh2d(meshes.projectile()),
            MeshMaterial2d(materials.projectile().clone()),
            transform.with_velocity(v.x, v.y),
            Transform::default(),
            Collision::new(1., CollisionLayer::Projectile, 40.),
            CollisionTimer::new(0.1),
        ));
    }
}
