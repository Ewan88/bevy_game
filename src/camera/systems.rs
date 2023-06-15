use bevy::input::mouse::*;

use crate::prelude::*;

const MOUSE_SENSITIVITY: f32 = 1.0;
const MOUSE_SENSITIVITY_SCALE: f32 = 1.0;

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
    mouse_motion: Res<Events<MouseMotion>>,
    mut camera_query: Query<&mut Transform, With<GameCamera>>,
) {
    if let Ok(mut camera_transform) = camera_query.get_single_mut() {
        if mouse_input.pressed(MouseButton::Middle) {
            for event in mouse_motion.get_reader().iter(&mouse_motion) {
                let delta = Vec3::new(
                    -MOUSE_SENSITIVITY
                        * MOUSE_SENSITIVITY_SCALE
                        * event.delta.x,
                    MOUSE_SENSITIVITY * MOUSE_SENSITIVITY_SCALE * event.delta.y,
                    0.0,
                );
                camera_transform.translation += delta;
            }
        }
    }
}

pub fn zoom_camera(
    mouse_wheel: Res<Events<MouseWheel>>,
    mut camera_query: Query<&mut OrthographicProjection, With<GameCamera>>,
) {
}
