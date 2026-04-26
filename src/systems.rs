use bevy::prelude::*;

use crate::{
    components::{Collision, LocalTransform},
    math::get_collision_velocities,
    player::entities::spawn_player,
    resources::WindowState,
};

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    scale: Res<WindowState>,
) {
    commands.spawn(Camera2d);

    let background = meshes.add(Rectangle::new(scale.width, scale.height));
    let background_color = Color::hsl(120. as f32 as f32, 0.05, 0.07);

    commands.spawn((
        Mesh2d(background),
        MeshMaterial2d(materials.add(background_color)),
        Transform::default(),
        LocalTransform::from_xyz(0., 0., -1.),
    ));
    spawn_player(&mut commands, &mut meshes, &mut materials, 50., 1., 0.9);
}
pub fn handle_window(
    mut window_state: ResMut<WindowState>,
    window: Single<&mut Window>,
    mut cursor_moved_reader: MessageReader<CursorMoved>,
) {
    window_state.scale = (window.width() / window_state.width)
        .min(window.height() / window_state.height);
    for cursor_moved in cursor_moved_reader.read() {
        window_state.cursor = vec2(
            cursor_moved.position.x - window.width() / 2.,
            window.height() / 2. - cursor_moved.position.y,
        ) / window_state.scale;
    }
}

pub fn local_to_global_transform(
    mut query: Query<(&LocalTransform, &mut Transform)>,
    window_state: Res<WindowState>,
) {
    for (local_transform, mut transform) in &mut query {
        transform.translation = vec3(
            local_transform.position.x,
            local_transform.position.y,
            local_transform.zindex,
        ) * window_state.scale;
        transform.rotation = Quat::from_rotation_z(local_transform.angle);
        transform.scale =
            vec3(local_transform.scale.x, local_transform.scale.y, 1.)
                * window_state.scale;
    }
}

pub fn advance_local_transform(
    mut query: Query<&mut LocalTransform>,
    time: Res<Time>,
) {
    for mut transform in &mut query {
        let a = transform.acceleration;
        let v = transform.velocity;
        transform.velocity += a * time.delta_secs();
        transform.position += v * time.delta_secs();
    }
}
pub fn handle_collision(
    mut query: Query<(&mut LocalTransform, &mut Collision)>,
) {
    let mut combinations = query.iter_combinations_mut();
    while let Some(
        [
            (mut transform_1, collision_1),
            (mut transform_2, collision_2),
        ],
    ) = combinations.fetch_next()
    {
        let d2 = transform_1.position.distance_squared(transform_2.position);
        if d2 > (collision_1.radius + collision_2.radius).powi(2) {
            continue;
        }
        let vd = transform_2.position.xy() - transform_1.position.xy();
        let (v1, v2) = get_collision_velocities(
            transform_1.velocity,
            transform_2.velocity,
            vd.to_angle(),
        );
        transform_1.velocity = v1;
        transform_2.velocity = v2;
        let nudge = vd.normalize()
            * (d2.sqrt() - collision_1.radius - collision_2.radius);
        transform_1.position += nudge / 2.;
        transform_2.position -= nudge / 2.;
    }
}
pub fn handle_edge_collision(
    query: Query<(&mut LocalTransform, &mut Collision)>,
    window: Res<WindowState>,
) {
    for (mut transform, collision) in query {
        if transform.position.x + collision.radius > window.width / 2. {
            transform.velocity.x = -transform.velocity.x;
            transform.position.x = window.width / 2. - collision.radius;
        }
        if transform.position.x - collision.radius < -window.width / 2. {
            transform.velocity.x = -transform.velocity.x;
            transform.position.x = -window.width / 2. + collision.radius;
        }
        if transform.position.y + collision.radius > window.height / 2. {
            transform.velocity.y = -transform.velocity.y;
            transform.position.y = window.height / 2. - collision.radius;
        }
        if transform.position.y - collision.radius < -window.height / 2. {
            transform.velocity.y = -transform.velocity.y;
            transform.position.y = -window.height / 2. + collision.radius;
        }
    }
}
