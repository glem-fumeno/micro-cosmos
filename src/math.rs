use std::f32::consts::PI;

use bevy::prelude::*;

pub fn get_collision_velocities(
    v1: Vec2,
    v2: Vec2,
    angle: f32,
) -> (Vec2, Vec2) {
    let t1 = v1.to_angle();
    let v1 = v1.length();
    let t2 = v2.to_angle();
    let v2 = v2.length();
    let ct1a = (t1 - angle).cos();
    let ct2a = (t2 - angle).cos();
    let st1a = (t1 - angle).sin();
    let st2a = (t2 - angle).sin();

    let v1x = v2 * ct2a * angle.cos() + v1 * st1a * (angle + PI / 2.).cos();
    let v1y = v2 * ct2a * angle.sin() + v1 * st1a * (angle + PI / 2.).sin();

    let v2x = v1 * ct1a * angle.cos() + v2 * st2a * (angle + PI / 2.).cos();
    let v2y = v1 * ct1a * angle.sin() + v2 * st2a * (angle + PI / 2.).sin();

    (vec2(v1x, v1y), vec2(v2x, v2y))
}
