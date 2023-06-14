use bevy::input::mouse::MouseWheel;

use crate::prelude::*;

#[derive(Component)]
pub struct GameCamera;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(
                DISPLAY_WIDTH / 2.0,
                DISPLAY_HEIGHT / 2.0,
                999.0,
            ),
            ..default()
        },
        GameCamera,
        RenderLayers::from_layers(&[0, 1]),
    ));
}

pub fn bound_camera() {}

pub fn move_camera(
    mouse_input: Res<Input<MouseButton>>,
    mut camera_query: Query<&mut Transform, With<GameCamera>>,
) {
}

pub fn zoom_camera(
    mouse_wheel: Res<Events<MouseWheel>>,
    mut camera_query: Query<&mut OrthographicProjection, With<GameCamera>>,
) {
}
