use std::f32::consts::TAU;

use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    platform::collections::HashMap,
    prelude::*,
};

use crate::{
    components::{
        Collision, CollisionLayer, CollisionTimer, FPSCounter, Health,
        LocalTransform, SceneEntity, TTL,
    },
    enemies::components::{Enemy, SpawnTimer},
    math::get_collision_velocities,
    player::entities::spawn_player,
    resources::{Materials, Meshes, Rng, Sounds, Textures, WindowState},
};
pub fn init_app(mut commands: Commands) {
    commands.spawn(Camera2d);
}
pub fn deinit_scene(
    mut commands: Commands,
    entities: Query<Entity, With<SceneEntity>>,
) {
    for entity in entities {
        commands.entity(entity).try_despawn();
    }
}

pub fn init_game(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials_resource: ResMut<Materials>,
    mut meshes_resource: ResMut<Meshes>,
    textures: Res<Textures>,
    scale: Res<WindowState>,
) {
    let background = meshes.add(Rectangle::new(scale.width, scale.height));

    commands.spawn((
        SceneEntity,
        Mesh2d(background),
        MeshMaterial2d(materials_resource.background.clone()),
        Transform::default(),
        LocalTransform::from_xyz(0., 0., -1.),
    ));
    spawn_player(
        &mut commands,
        &mut meshes_resource,
        &mut materials_resource,
        &textures,
        50.,
        0.5,
        0.4,
    );
    commands.spawn((
        SceneEntity,
        SpawnTimer(Timer::from_seconds(1., TimerMode::Once)),
    ));
    commands.spawn((
        SceneEntity,
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
    mut commands: Commands,
    mut query: Query<(Entity, &mut LocalTransform, &mut Collision)>,
    mut collision_query: Query<(&mut CollisionTimer, &mut Sprite, &Enemy)>,
    window_state: Res<WindowState>,
    textures: Res<Textures>,
    sounds: Res<Sounds>,
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
    let mut updated_entities: HashMap<Entity, (Vec2, Vec2, f32)> =
        HashMap::new();
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
            let mut updated = false;
            let f1 = (
                transform_1.velocity,
                transform_1.position,
                collision_1.energy,
            );
            let (v1, p1, e1) = updated_entities.get(entity_1).unwrap_or(&f1);
            let (mut v1, mut p1, mut e1) = (*v1, *p1, *e1);
            let (r1, m1) = (collision_1.radius, collision_1.mass);

            for (entity_2, transform_2, collision_2) in
                entities.iter().skip(i + 1)
            {
                if !collision_2.layer.collides_with(collision_1.layer) {
                    continue;
                }
                let f2 = (
                    transform_2.velocity,
                    transform_2.position,
                    collision_1.energy,
                );
                let (v2, p2, e2) =
                    updated_entities.get(entity_2).unwrap_or(&f2);
                let (mut v2, mut p2, mut e2) = (*v2, *p2, *e2);
                let (r2, m2) = (collision_2.radius, collision_2.mass);

                if p1.distance_squared(p2) > (r1 + r2) * (r1 + r2) {
                    continue;
                }
                updated = true;
                let dp = p2 - p1;
                let (nv1, nv2) =
                    get_collision_velocities(v1, v2, m1, m2, dp.to_angle());
                e1 += (nv1 - v1).length_squared() * m1;
                e2 += (nv2 - v2).length_squared() * m2;
                v1 = nv1;
                v2 = nv2;
                let nudge = dp.normalize() * (p1.distance(p2) - r1 - r2);
                (p1, p2) = (p1 + nudge / 2., p2 - nudge / 2.);
                updated_entities.insert(*entity_2, (v2, p2, e2));
            }
            if updated {
                updated_entities.insert(*entity_1, (v1, p1, e1));
            }
        }
    }
    let mut played = false;
    for (entity, (velocity, position, energy)) in updated_entities {
        if let Ok((_, mut transform, mut collision)) = query.get_mut(entity) {
            transform.velocity = velocity;
            transform.position = position;
            collision.energy = energy
        }
        if let Ok((mut collision, mut sprite, enemy)) =
            collision_query.get_mut(entity)
        {
            if collision.timer.is_finished() {
                collision.texture = Some(sprite.image.clone());
                sprite.image = textures.enemyd_from_index(enemy.index).unwrap();
                if !played && sounds.now_playing < 2 {
                    commands.spawn((
                        SceneEntity,
                        AudioPlayer::new(sounds.pop.clone()),
                        PlaybackSettings::DESPAWN,
                    ));
                    played = true;
                }
            }
            collision.timer.reset();
        }
    }
}
pub fn handle_edge_collision(
    query: Query<(&mut LocalTransform, &Collision)>,
    window: Res<WindowState>,
) {
    for (mut transform, collision) in query {
        let mut new_velocity = transform.velocity;
        if transform.position.x + collision.radius > window.width / 2. {
            new_velocity.x = -new_velocity.x.abs();
            transform.position.x = window.width / 2. - collision.radius;
        }
        if transform.position.x - collision.radius < -window.width / 2. {
            new_velocity.x = new_velocity.x.abs();
            transform.position.x = -window.width / 2. + collision.radius;
        }
        if transform.position.y + collision.radius > window.height / 2. {
            new_velocity.y = -new_velocity.y.abs();
            transform.position.y = window.height / 2. - collision.radius;
        }
        if transform.position.y - collision.radius < -window.height / 2. {
            new_velocity.y = new_velocity.y.abs();
            transform.position.y = -window.height / 2. + collision.radius;
        }
        transform.velocity = new_velocity;
    }
}
pub fn handle_fps_count(
    mut text: Single<&mut Text, With<FPSCounter>>,
    diagnostics: Res<DiagnosticsStore>,
) {
    if let Some(diagnostic) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS)
        && let Some(fps) = diagnostic.smoothed()
    {
        text.0 = format!("FPS: {}", fps as i32).into()
    }
}
pub fn handle_ttl(
    mut commands: Commands,
    mut query: Query<(Entity, &mut TTL)>,
    mut transform_query: Query<&mut LocalTransform>,
    time: Res<Time>,
) {
    for (entity, mut timer) in &mut query {
        if timer.tick(time.delta()).just_finished() {
            commands.entity(entity).despawn();
        }
        if let Ok(mut transform) = transform_query.get_mut(entity) {
            transform.scale =
                Vec2::ONE * (timer.fraction_remaining() + 0.5) / 1.5;
        }
    }
}
pub fn handle_collision_time(
    query: Query<(&mut CollisionTimer, &mut Sprite)>,
    time: Res<Time>,
) {
    for (mut collision, mut sprite) in query {
        if collision.timer.tick(time.delta()).just_finished() {
            if let Some(ctexture) = &collision.texture {
                sprite.image = ctexture.clone();
            }
            collision.texture = None;
        }
    }
}
pub fn handle_health(
    mut commands: Commands,
    mut query: Query<(Entity, &Collision, &Health)>,
    transform_query: Query<&LocalTransform>,
    mut rng: ResMut<Rng>,
    textures: Res<Textures>,
    sounds: Res<Sounds>,
) {
    let mut played = false;
    for (entity, collision, health) in &mut query {
        if collision.energy > health.health * 1_000_000. {
            commands.entity(entity).despawn();
            if !played && sounds.now_playing < 2 {
                commands.spawn((
                    SceneEntity,
                    AudioPlayer::new(sounds.click.clone()),
                    PlaybackSettings::DESPAWN,
                ));
                played = true;
            }
            if let Ok(transform) = transform_query.get(entity) {
                for _ in 0..5 {
                    let angle = rng.random_to(TAU);
                    let v = Vec2::from_angle(angle) * 100.;
                    commands.spawn((
                        SceneEntity,
                        TTL(Timer::from_seconds(0.2, TimerMode::Once)),
                        Sprite::from_image(textures.particle.clone()),
                        transform.with_velocity(v.x, v.y),
                        Transform::default(),
                        Collision::new(1., CollisionLayer::Projectile, 40.),
                        CollisionTimer::new(0.1),
                    ));
                }
            }
        }
    }
}
pub fn handle_play_limit(
    query: Query<&AudioPlayer>,
    mut sounds: ResMut<Sounds>,
) {
    sounds.now_playing = query.count();
}
