use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_player)
        .add_systems(Startup, spawn_cam)
        .add_systems(Update, handle_movement)
        .add_systems(Update, confine_player)
        .run();
}

#[derive(Component)]
pub struct Player;

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
