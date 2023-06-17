use bevy::{input::mouse::*, render::camera::ScalingMode};

use crate::prelude::*;

const MOUSE_SENSITIVITY: f32 = 0.1;
const MOUSE_SENSITIVITY_SCALE: f32 = 0.2;

#[derive(Component)]
pub struct GameCamera;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            projection: OrthographicProjection {
                scale: 3.0,
                scaling_mode: ScalingMode::FixedVertical(2.0),
                near: 0.0,
                ..default()
            }
            .into(),
            transform: Transform {
                translation: Vec3::new(100.0, 100.0, 100.0),
                scale: Vec3::splat(2.0),
                ..default()
            }
            .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        FogSettings {
            color: Color::rgba(0.1, 0.1, 0.1, 1.0),
            falloff: FogFalloff::Linear {
                start: 100.0,
                end: 200.0,
            },
            ..default()
        },
        GameCamera,
    ));
}

pub fn move_camera(
    mouse_input: Res<Input<MouseButton>>,
    mouse_motion: Res<Events<MouseMotion>>,
    mut camera_query: Query<&mut Transform, With<GameCamera>>,
) {
    let Ok(mut camera_transform) = camera_query.get_single_mut()
        else { return; };
    if mouse_input.pressed(MouseButton::Middle) {
        for event in mouse_motion.get_reader().iter(&mouse_motion) {
            let delta = Vec3::new(
                -MOUSE_SENSITIVITY * MOUSE_SENSITIVITY_SCALE * event.delta.x,
                MOUSE_SENSITIVITY * MOUSE_SENSITIVITY_SCALE * event.delta.y,
                0.0,
            );
            camera_transform.translation += delta;
        }
    }
}

pub fn zoom_camera(
    mouse_wheel: Res<Events<MouseWheel>>,
    mut camera_query: Query<&mut Transform, With<GameCamera>>,
) {
    let mut transform = camera_query.single_mut();
    for event in mouse_wheel.get_reader().iter(&mouse_wheel) {
        let zoom = 1.0 - event.y * 0.2;
        transform.scale *= Vec3::splat(zoom);
        transform.scale =
            transform.scale.clamp(Vec3::splat(2.0), Vec3::splat(20.0));
    }
}
