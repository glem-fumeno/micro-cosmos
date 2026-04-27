use std::f32::consts::PI;

use bevy::prelude::*;

pub fn get_collision_velocities(
    v1: Vec2,
    v2: Vec2,
    r1: f32,
    r2: f32,
    angle: f32,
) -> (Vec2, Vec2) {
    let t1 = v1.to_angle();
    let v1 = v1.length();
    let m1 = r1 * r1;
    let t2 = v2.to_angle();
    let v2 = v2.length();
    let m2 = r2 * r2;
    let ms = m1 + m2;
    let aco = angle.cos();
    let asi = angle.sin();
    let bco = (angle + PI / 2.).cos();
    let bsi = (angle + PI / 2.).sin();
    let ct1a = (t1 - angle).cos();
    let ct2a = (t2 - angle).cos();
    let st1a = (t1 - angle).sin();
    let st2a = (t2 - angle).sin();

    let m1c = v1 * ct1a * (m1 - m2);
    let v1x = (m1c + 2. * m2 * v2 * ct2a * aco) / ms + v1 * st1a * bco;
    let v1y = (m1c + 2. * m2 * v2 * ct2a * asi) / ms + v1 * st1a * bsi;

    let m2c = v2 * ct2a * (m2 - m1);
    let v2x = (m2c + 2. * m1 * v1 * ct1a * aco) / ms + v2 * st2a * bco;
    let v2y = (m2c + 2. * m2 * v1 * ct1a * asi) / ms + v2 * st2a * bsi;

    (vec2(v1x, v1y), vec2(v2x, v2y))
}
