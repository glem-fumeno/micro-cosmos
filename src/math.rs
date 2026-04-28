use std::f32::consts::PI;

use bevy::prelude::*;

pub fn get_collision_velocities(
    v1: Vec2,
    v2: Vec2,
    m1: f32,
    m2: f32,
    angle: f32,
) -> (Vec2, Vec2) {
    let t1 = v1.to_angle();
    let v1 = v1.length();
    let t2 = v2.to_angle();
    let v2 = v2.length();
    let ms = m1 + m2;
    let cos_a = angle.cos();
    let sin_a = angle.sin();
    let cos_b = (angle + PI / 2.).cos();
    let sin_b = (angle + PI / 2.).sin();
    let cos_t1a = (t1 - angle).cos();
    let cos_t2a = (t2 - angle).cos();
    let sin_t1a = (t1 - angle).sin();
    let sin_t2a = (t2 - angle).sin();

    let num_1 = v1 * cos_t1a * (m1 - m2) + 2. * m2 * v2 * cos_t2a;
    let v1x = num_1 * cos_a / ms + v1 * sin_t1a * cos_b;
    let v1y = num_1 * sin_a / ms + v1 * sin_t1a * sin_b;

    let num_2 = v2 * cos_t2a * (m2 - m1) + 2. * m1 * v1 * cos_t1a;
    let v2x = num_2 * cos_a / ms + v2 * sin_t2a * cos_b;
    let v2y = num_2 * sin_a / ms + v2 * sin_t2a * sin_b;

    (vec2(v1x, v1y), vec2(v2x, v2y))
}
