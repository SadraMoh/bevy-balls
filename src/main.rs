use bevy::prelude::*;
use rand::{random, seq::SliceRandom, thread_rng, Rng};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_cam)
        .add_systems(Startup, spawn_player)
        .add_systems(Startup, spawn_enemies)
        .add_systems(Startup, spawn_stars)
        .add_systems(Update, handle_movement)
        .add_systems(Update, confine_player)
        .add_systems(Update, enemy_movement)
        .add_systems(Update, confine_enemy)
        .add_systems(Update, kill_player)
        .add_systems(Update, eat_star)
        .run();
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Star;

#[derive(Component)]
pub struct Enemy {
    direction: Vec3,
}

pub fn spawn_player(mut commands: Commands, windows: Query<&Window>, assets: Res<AssetServer>) {
    let window = windows.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2., window.height() / 2., 0.),
            texture: assets.load("img/ball_blue_large.png"),
            ..default()
        },
        Player {},
    ));
}

pub fn spawn_stars(mut commands: Commands, windows: Query<&Window>, assets: Res<AssetServer>) {
    let window = windows.get_single().unwrap();

    let mut thread_rng = rand::thread_rng();

    for _ in 0..thread_rng.gen_range(1..3) {
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(
                    thread_rng.gen_range(PLAYER_SIZE..window.width() - PLAYER_SIZE),
                    thread_rng.gen_range(PLAYER_SIZE..(window.height() - PLAYER_SIZE) / 3.),
                    0.,
                ),
                texture: assets.load("img/star.png"),
                ..default()
            },
            Star {},
        ));
    }
}

pub fn spawn_enemies(mut commands: Commands, windows: Query<&Window>, assets: Res<AssetServer>) {
    let window = windows.get_single().unwrap();

    let mut thread_rng = rand::thread_rng();

    for _ in 0..thread_rng.gen_range(1..3) {
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(
                    thread_rng.gen_range(PLAYER_SIZE..window.width() - PLAYER_SIZE),
                    thread_rng.gen_range(PLAYER_SIZE..(window.height() - PLAYER_SIZE) / 3.),
                    0.,
                ),
                texture: assets.load("img/ball_red_large.png"),
                ..default()
            },
            Enemy {
                direction: Vec3::new(random(), random(), 0.).normalize(),
            },
        ));
    }
}

pub fn enemy_movement(mut enemies: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut transform, enemy) in enemies.iter_mut() {
        transform.translation += enemy.direction * time.delta_seconds() * PLAYER_SPEED;
    }
}

pub fn spawn_cam(mut commands: Commands, windows: Query<&Window>) {
    let window = windows.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2., window.height() / 2., 0.),
        ..default()
    });
}

const PLAYER_SPEED: f32 = 500.;

pub fn handle_movement(
    keyboard: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let Ok(mut player_transform) = player_query.get_single_mut() else {
        return;
    };

    let mut direction: Vec3 = Vec3::ZERO;

    if keyboard.pressed(KeyCode::Left) {
        direction += Vec3::NEG_X;
    }

    if keyboard.pressed(KeyCode::Right) {
        direction += Vec3::X;
    }

    if keyboard.pressed(KeyCode::Up) {
        direction += Vec3::Y;
    }

    if keyboard.pressed(KeyCode::Down) {
        direction += Vec3::NEG_Y;
    }

    player_transform.translation +=
        PLAYER_SPEED * direction.normalize_or_zero() * time.delta_seconds();
}

const PLAYER_SIZE: f32 = 64.;

pub fn confine_player(
    mut player_query: Query<&mut Transform, With<Player>>,
    window: Query<&Window>,
) {
    let Ok(mut player_transform) = player_query.get_single_mut() else {
        return;
    };

    let window = window.get_single().unwrap();

    const HALF: f32 = PLAYER_SIZE / 2.;

    if player_transform.translation.x <= HALF {
        player_transform.translation.x = HALF;
    }

    if player_transform.translation.x >= window.width() - HALF {
        player_transform.translation.x = window.width() - HALF;
    }

    if player_transform.translation.y <= HALF {
        player_transform.translation.y = HALF;
    }

    if player_transform.translation.y >= window.height() - HALF {
        player_transform.translation.y = window.height() - HALF;
    }
}

pub fn confine_enemy(
    mut commands: Commands,
    mut enemies: Query<(&mut Transform, &mut Enemy)>,
    window: Query<&Window>,
    assets: Res<AssetServer>,
) {
    let window = window.get_single().unwrap();

    const HALF: f32 = PLAYER_SIZE / 1.8;

    for (mut transform, mut enemy) in enemies.iter_mut() {
        let mut has_bumped = true;

        match (transform.translation.x, transform.translation.y) {
            (x, _) if x < HALF || x > window.width() - HALF => {
                enemy.direction.x *= -1.;
                transform.translation += enemy.direction * 8.;
            }
            (_, y) if y < HALF || y > window.height() - HALF => {
                enemy.direction.y *= -1.;
                transform.translation += enemy.direction * 8.;
            }
            _ => has_bumped = false,
        }

        if has_bumped {
            let sounds: [Handle<AudioSource>; 5] = [
                assets.load("audio/impactMining_000.ogg"),
                assets.load("audio/impactMining_001.ogg"),
                assets.load("audio/impactMining_002.ogg"),
                assets.load("audio/impactMining_003.ogg"),
                assets.load("audio/impactMining_004.ogg"),
            ];

            let sound = sounds.choose(&mut thread_rng()).unwrap().clone();

            commands.spawn(AudioBundle {
                source: sound,
                settings: PlaybackSettings {
                    mode: bevy::audio::PlaybackMode::Despawn,
                    ..default()
                },
            });
        }
    }
}

pub fn eat_star(
    mut commands: Commands,
    stars: Query<(Entity, &Transform), With<Star>>,
    player_query: Query<&Transform, With<Player>>,
    assets: Res<AssetServer>,
) {
    let Ok(player_transform) = player_query.get_single() else {
        return;
    };

    for (star_entity, star_transform) in stars.iter() {
        if star_transform
            .translation
            .distance(player_transform.translation)
            <= PLAYER_SIZE
        {
            let sounds: [Handle<AudioSource>; 2] = [
                assets.load("audio/pluck_000.ogg"),
                assets.load("audio/pluck_001.ogg"),
            ];

            let sound = sounds.choose(&mut thread_rng()).unwrap().clone();

            commands.spawn(AudioBundle {
                source: sound,
                settings: PlaybackSettings {
                    mode: bevy::audio::PlaybackMode::Despawn,
                    ..default()
                },
            });

            commands.entity(star_entity).despawn();
        }
    }
}

pub fn kill_player(
    mut commands: Commands,
    enemies: Query<&Transform, With<Enemy>>,
    player_query: Query<(Entity, &Transform), With<Player>>,
    assets: Res<AssetServer>,
) {
    let Ok((player_entity, player_transform)) = player_query.get_single() else {
        return;
    };

    for enemy_transform in enemies.iter() {
        if enemy_transform
            .translation
            .distance(player_transform.translation)
            <= PLAYER_SIZE
        {
            let sounds: [Handle<AudioSource>; 2] = [
                assets.load("audio/explosionCrunch_000.ogg"),
                assets.load("audio/explosionCrunch_001.ogg"),
            ];

            let sound = sounds.choose(&mut thread_rng()).unwrap().clone();

            commands.spawn(AudioBundle {
                source: sound,
                settings: PlaybackSettings {
                    mode: bevy::audio::PlaybackMode::Despawn,
                    ..default()
                },
            });

            commands.entity(player_entity).despawn();
        }
    }
}
