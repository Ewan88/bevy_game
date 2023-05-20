use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::components::Player;

pub const PLAYER_SPEED: f32 = 100.0;
pub const PLAYER_SIZE: f32 = 32.0;

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    let transform = Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0);
    commands.spawn((
        SpriteBundle {
            transform,
            texture: asset_server.load("sprites/player/player.png"),
            ..default()
        },
        Player { destination: None },
    ));
}

pub fn despawn_player(mut commands: Commands, player_query: Query<Entity, With<Player>>) {
    if let Ok(player_entity) = player_query.get_single() {
        commands.entity(player_entity).despawn();
    }
}

pub fn move_player(
    mouse_input: Res<Input<MouseButton>>,
    mut player_query: Query<&mut Player>,
    window: Query<&Window>,
) {
    if let Ok(mut player) = player_query.get_single_mut() {
        let window: &Window = window.single();
        if let Some(destination) = window.cursor_position() {
            if mouse_input.pressed(MouseButton::Right) {
                player.destination = Some(destination);
            }
        }
    }
}

pub fn update_player(
    mut transform_query: Query<&mut Transform, With<Player>>,
    mut player_query: Query<&mut Player>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = transform_query.get_single_mut() {
        if let Ok(player) = player_query.get_single_mut() {
            match player.destination {
                None => {
                    // do nothing
                }
                Some(destination) => {
                    let mut translation = transform.translation;
                    let mut direction: Vec3 = Vec3 {
                        x: transform.translation.x + destination.x,
                        y: transform.translation.y + destination.y,
                        z: 0.0,
                    };

                    if direction.length() > 0.0 {
                        direction = direction.normalize();
                    }

                    if translation.x < destination.x {
                        translation.x += direction.x * PLAYER_SPEED * time.delta_seconds();
                    } else if translation.x > destination.x {
                        translation.x -= direction.x * PLAYER_SPEED * time.delta_seconds();
                    }

                    if translation.y < destination.y {
                        translation.y += direction.y * PLAYER_SPEED * time.delta_seconds();
                    } else if translation.y > destination.y {
                        translation.y -= direction.y * PLAYER_SPEED * time.delta_seconds();
                    }

                    transform.translation = translation;
                }
            }
        }
    }
}

pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();
        let x_min = 0.0 + PLAYER_SIZE;
        let x_max = window.width() - PLAYER_SIZE;
        let y_min = 0.0 + PLAYER_SIZE;
        let y_max = window.height() - PLAYER_SIZE;

        let mut translation = player_transform.translation;

        // Bound the x position
        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }
        // Bound the y position
        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        player_transform.translation = translation;
    }
}
