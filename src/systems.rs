use std::time::Duration;

use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    platform::collections::HashMap,
    prelude::*,
};

use crate::{
    components::{Collision, FPSCounter, LocalTransform, TTL},
    enemies::components::SpawnTimer,
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
    commands.spawn((SpawnTimer(Timer::from_seconds(1., TimerMode::Once)),));
    commands.spawn((
        Text::new(String::new()),
        Node {
            position_type: PositionType::Absolute,
            top: px(12),
            left: px(12),
            ..default()
        },
        FPSCounter,
    ));
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
    mut query: Query<(Entity, &mut LocalTransform, &mut Collision)>,
    window_state: Res<WindowState>,
) {
    let mut grid: HashMap<
        (i32, i32),
        Vec<(Entity, LocalTransform, Collision)>,
    > = HashMap::new();
    for (entity, transform, collision) in &query {
        let sector_x = transform.position.x as i32 / window_state.sector_size;
        let sector_y = transform.position.y as i32 / window_state.sector_size;
        grid.entry((sector_x, sector_y))
            .and_modify(|v| v.push((entity, *transform, *collision)))
            .or_insert(vec![(entity, *transform, *collision)]);
    }
    let mut updated_entities: HashMap<Entity, (Vec2, Vec2)> = HashMap::new();
    for (sx, sy) in grid.keys() {
        let mut entities: Vec<(Entity, LocalTransform, Collision)> = vec![];
        entities.extend(grid.get(&(sx - 1, sy - 1)).unwrap_or(&vec![]));
        entities.extend(grid.get(&(sx - 0, sy - 1)).unwrap_or(&vec![]));
        entities.extend(grid.get(&(sx + 1, sy - 1)).unwrap_or(&vec![]));
        entities.extend(grid.get(&(sx - 1, sy - 0)).unwrap_or(&vec![]));
        entities.extend(grid.get(&(sx - 0, sy - 0)).unwrap_or(&vec![]));
        entities.extend(grid.get(&(sx + 1, sy - 0)).unwrap_or(&vec![]));
        entities.extend(grid.get(&(sx - 1, sy + 1)).unwrap_or(&vec![]));
        entities.extend(grid.get(&(sx - 0, sy + 1)).unwrap_or(&vec![]));
        entities.extend(grid.get(&(sx + 1, sy + 1)).unwrap_or(&vec![]));
        for (i, (entity_1, transform_1, collision_1)) in
            entities.iter().enumerate()
        {
            let f1 = (transform_1.velocity, transform_1.position);
            let (v1, p1) = updated_entities.get(entity_1).unwrap_or(&f1);
            let (mut v1, mut p1) = (*v1, *p1);
            let (r1, m1) = (collision_1.radius, collision_1.mass);

            for (entity_2, transform_2, collision_2) in
                entities.iter().skip(i + 1)
            {
                if !collision_2.layer.collides_with(collision_1.layer) {
                    continue;
                }
                let f2 = (transform_2.velocity, transform_2.position);
                let (v2, p2) = updated_entities.get(entity_2).unwrap_or(&f2);
                let (mut v2, mut p2) = (*v2, *p2);
                let (r2, m2) = (collision_2.radius, collision_2.mass);

                if p1.distance_squared(p2) > (r1 + r2) * (r1 + r2) {
                    continue;
                }
                let dp = p2 - p1;
                (v1, v2) =
                    get_collision_velocities(v1, v2, m1, m2, dp.to_angle());
                let nudge = dp.normalize() * (p1.distance(p2) - r1 - r2);
                (p1, p2) = (p1 + nudge / 2., p2 - nudge / 2.);
                updated_entities.insert(*entity_2, (v2, p2));
            }
            updated_entities.insert(*entity_1, (v1, p1));
        }
    }
    for (entity, (velocity, position)) in updated_entities {
        if let Ok((_, mut transform, _)) = query.get_mut(entity) {
            transform.velocity = velocity;
            transform.position = position;
        }
    }
}
pub fn handle_edge_collision(
    query: Query<(&mut LocalTransform, &mut Collision)>,
    window: Res<WindowState>,
) {
    for (mut transform, collision) in query {
        if transform.position.x + collision.radius > window.width / 2. {
            transform.velocity.x = -transform.velocity.x.abs();
            transform.position.x = window.width / 2. - collision.radius;
        }
        if transform.position.x - collision.radius < -window.width / 2. {
            transform.velocity.x = transform.velocity.x.abs();
            transform.position.x = -window.width / 2. + collision.radius;
        }
        if transform.position.y + collision.radius > window.height / 2. {
            transform.velocity.y = -transform.velocity.y.abs();
            transform.position.y = window.height / 2. - collision.radius;
        }
        if transform.position.y - collision.radius < -window.height / 2. {
            transform.velocity.y = transform.velocity.y.abs();
            transform.position.y = -window.height / 2. + collision.radius;
        }
    }
}
pub fn handle_fps_count(
    mut query: Query<&mut Text, With<FPSCounter>>,
    diagnostics: Res<DiagnosticsStore>,
) {
    if let Some(diagnostic) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS)
        && let Some(fps) = diagnostic.smoothed()
    {
        for mut text in &mut query {
            text.0 = format!("FPS: {}", fps as i32).into()
        }
    }
}
pub fn handle_ttl(
    mut commands: Commands,
    mut query: Query<(Entity, &mut TTL)>,
    time: Res<Time>,
) {
    for (entity, mut timer) in &mut query {
        if timer.tick(time.delta()).just_finished() {
            let last_duration = timer.0.duration().as_secs_f64();
            timer.set_duration(Duration::from_secs_f64(last_duration * 0.99));
            timer.reset();
            commands.entity(entity).despawn();
        }
    }
}
