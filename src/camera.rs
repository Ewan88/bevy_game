use crate::prelude::*;

pub fn setup_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(
                window.width() / 2.0,
                window.height() / 2.0,
                999.0,
            ),
            ..default()
        },
        RenderLayers::from_layers(&[0, 1]),
    ));
}
