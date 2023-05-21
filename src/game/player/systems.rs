use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::components::*;

pub const PLAYER_SPEED: f32 = 200.0;
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
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mouse_input: Res<Input<MouseButton>>,
    mut player_query: Query<&mut Player>,
    window: Query<&Window>,
    icon_query: Query<Entity, With<MoveIcon>>,
) {
    if let Ok(mut player) = player_query.get_single_mut() {
        let window: &Window = window.single();
        if let Some(destination) = window.cursor_position() {
            if mouse_input.pressed(MouseButton::Right) {
                player.destination = Some(destination);
                if let Ok(move_icon) = icon_query.get_single() {
                    commands.entity(move_icon).despawn();
                }
                commands.spawn((
                    SpriteBundle {
                        transform: Transform::from_xyz(destination.x, destination.y, 0.0),
                        texture: asset_server.load("ui/move_marker.png"),
                        ..default()
                    },
                    MoveIcon {},
                ));
                println!("{}", destination);
            }
        }
    }
}

pub fn update_player(
    mut transform_query: Query<&mut Transform, With<Player>>,
    mut player_query: Query<&mut Player>,
    time: Res<Time>,
    mut commands: Commands,
    icon_query: Query<Entity, With<MoveIcon>>,
) {
    if let Ok(mut transform) = transform_query.get_single_mut() {
        if let Ok(player) = player_query.get_single_mut() {
            match player.destination {
                None => {
                    // do nothing
                }
                Some(destination) => {
                    let dest_x = destination.x.round();
                    let dest_y = destination.y.round();

                    let mut translation = transform.translation;
                    let trans_x = translation.x.round();
                    let trans_y = translation.y.round();

                    if let Ok(move_icon) = icon_query.get_single() {
                        if trans_x == dest_x && trans_y == dest_y {
                            commands.entity(move_icon).despawn();
                        }
                    }

                    let mut direction: Vec3 = Vec3 {
                        x: transform.translation.x + destination.x,
                        y: transform.translation.y + destination.y,
                        z: 0.0,
                    };

                    if direction.length() > 0.0 {
                        direction = direction.normalize();
                    }

                    if trans_x < dest_x {
                        translation.x += direction.x * PLAYER_SPEED * time.delta_seconds();
                    } else if trans_x > dest_x {
                        translation.x -= direction.x * PLAYER_SPEED * time.delta_seconds();
                    }

                    if trans_y < dest_y {
                        translation.y += direction.y * PLAYER_SPEED * time.delta_seconds();
                    } else if trans_y > dest_y {
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
