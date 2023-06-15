use super::components::*;
use super::resources::*;
use crate::camera::systems::GameCamera;
use crate::prelude::*;

pub const PLAYER_SPEED: f32 = 200.0; // TODO: Add acceleration
#[allow(dead_code)]
pub const PLAYER_SIZE: f32 = 32.0;

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let transform =
        Transform::from_xyz(DISPLAY_WIDTH / 2.0, DISPLAY_HEIGHT / 2.0, 1.0);
    commands.spawn((
        SpriteBundle {
            transform,
            texture: asset_server.load("sprites/player/player.png"),
            ..default()
        },
        RenderLayers::layer(1),
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
    camera_query: Query<(&Camera, &GlobalTransform), With<GameCamera>>,
) {
    if let Ok(mut player) = player_query.get_single_mut() {
        let Some(cursor_position) = window.single().cursor_position() else { return; };
        let (camera, camera_transform) = camera_query.single();
        if mouse_input.pressed(MouseButton::Right) {
            if let Ok(entity) = icon_query.get_single() {
                despawn.0.insert(entity);
            }
            let Some(cursor_world_position) = camera.viewport_to_world_2d(
                camera_transform, cursor_position) else { return; };

            if let Ok(transform) = transform_query.get_single() {
                if transform.translation.x != cursor_world_position.x
                    || transform.translation.y != cursor_world_position.y
                {
                    commands.spawn((
                        SpriteBundle {
                            transform: Transform::from_xyz(
                                cursor_world_position.x,
                                cursor_world_position.y,
                                1.0,
                            ),
                            texture: asset_server.load("ui/move_marker.png"),
                            ..default()
                        },
                        RenderLayers::layer(1),
                        MoveIcon {},
                    ));

                    player.destination = Some(cursor_world_position);
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

#[allow(unused_variables, unused_mut)]
pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    // use the game size to bound the player's movement
    // not the display size because the camera can move
    // if let Ok(mut player_transform) = player_query.get_single_mut() {
    //     let x_min = 0.0 + PLAYER_SIZE;
    //     let x_max = MAP_WIDTH - PLAYER_SIZE;
    //     let y_min = 0.0 + PLAYER_SIZE;
    //     let y_max = MAP_HEIGHT - PLAYER_SIZE;

    //     let mut translation = player_transform.translation;

    //     // Bound the x position
    //     if translation.x < x_min {
    //         translation.x = x_min;
    //     } else if translation.x > x_max {
    //         translation.x = x_max;
    //     }
    //     // Bound the y position
    //     if translation.y < y_min {
    //         translation.y = y_min;
    //     } else if translation.y > y_max {
    //         translation.y = y_max;
    //     }

    //     player_transform.translation = translation;
    // }
}
