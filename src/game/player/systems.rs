use crate::prelude::*;

use super::components::*;
use super::resources::*;

pub const PLAYER_SPEED: f32 = 200.0; // TODO: Add acceleration
pub const PLAYER_SIZE: f32 = 32.0;

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    let transform =
        Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0);
    commands.spawn((
        SpriteBundle {
            transform,
            texture: asset_server.load("sprites/player/player.png"),
            ..default()
        },
        Player { destination: None },
    ));
}

pub fn despawn_player(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
) {
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
    mut despawn: ResMut<DespawnSet>,
    icon_query: Query<Entity, With<MoveIcon>>,
    transform_query: Query<&Transform, With<Player>>,
) {
    if let Ok(mut player) = player_query.get_single_mut() {
        let window: &Window = window.single();
        if let Some(destination) = window.cursor_position() {
            if mouse_input.pressed(MouseButton::Right) {
                if let Ok(entity) = icon_query.get_single() {
                    despawn.0.insert(entity);
                }
                if let Ok(transform) = transform_query.get_single() {
                    if transform.translation.x != destination.x
                        || transform.translation.y != destination.y
                    {
                        commands.spawn((
                            SpriteBundle {
                                transform: Transform::from_xyz(
                                    destination.x,
                                    destination.y,
                                    0.0,
                                ),
                                texture: asset_server
                                    .load("ui/move_marker.png"),
                                ..default()
                            },
                            MoveIcon {},
                        ));

                        player.destination = Some(destination);
                    }
                }
            }
        }
    }
}

pub fn update_player(
    mut transform_query: Query<&mut Transform, With<Player>>,
    mut player_query: Query<&mut Player>,
    time: Res<Time>,
    mut despawn: ResMut<DespawnSet>,
    icon_query: Query<Entity, With<MoveIcon>>,
) {
    if let Ok(mut transform) = transform_query.get_single_mut() {
        if let Ok(mut player) = player_query.get_single_mut() {
            match player.destination {
                None => {}
                Some(destination) => {
                    let dest_x = destination.x;
                    let dest_y = destination.y;

                    let mut translation = transform.translation;
                    let trans_x = translation.x;
                    let trans_y = translation.y;

                    if trans_x == dest_x && trans_y == dest_y {
                        if let Ok(entity) = icon_query.get_single() {
                            despawn.0.insert(entity);
                        }
                        player.destination = None;
                    }

                    let direction =
                        Vec3::new(dest_x - trans_x, dest_y - trans_y, 0.0);
                    let distance = direction.length();

                    if distance > 0.0 {
                        let speed = PLAYER_SPEED * time.delta_seconds();
                        let movement = direction.normalize() * speed;

                        if distance <= speed {
                            translation.x = dest_x;
                            translation.y = dest_y;
                        } else {
                            translation.x += movement.x;
                            translation.y += movement.y;
                        }

                        transform.translation = translation;
                    }
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
