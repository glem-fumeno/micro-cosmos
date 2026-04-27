use std::f32::consts::PI;

use bevy::prelude::*;

use crate::{
    components::{Collision, CollisionLayer, LocalTransform, TTL},
    projectiles::components::Projectile,
};

pub fn spawn_projectiles(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    transform: LocalTransform,
    count: i32,
    spread: f32,
) {
    let mesh = meshes.add(Circle::new(1.0));
    let color = Color::hsl(90. as f32 as f32, 0.75, 0.7);
    let material = materials.add(color);
    for i in 0..count {
        let delta = (spread / (count as f32 - 1.)) * (i as f32);
        let angle = transform.angle - spread / 2. + delta;
        let v = Vec2::from_angle(angle + PI / 2.) * 100.;
        commands.spawn((
            Projectile { velocity: 100. },
            TTL(Timer::from_seconds(0.5, TimerMode::Once)),
            Mesh2d(mesh.clone()),
            MeshMaterial2d(material.clone()),
            transform.with_velocity(v.x, v.y),
            Transform::default(),
            Collision::new(1., CollisionLayer::Projectile),
        ));
    }
}
