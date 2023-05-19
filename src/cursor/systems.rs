use bevy::prelude::*;

use super::components::GameCursor;

pub fn spawn_menu_cursor(
    mut windows: Query<&mut Window>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let mut window: Mut<Window> = windows.single_mut();
    window.cursor.visible = false;
    let cursor_spawn: Vec3 = Vec3::ZERO;

    commands.spawn((
        ImageBundle {
            image: asset_server.load("cursors/point.png").into(),
            style: Style {
                //display: Display::None,
                position_type: PositionType::Absolute,
                position: UiRect::all(Val::Auto),
                size: Size::new(Val::Px(24.0), Val::Px(24.0)),
                ..default()
            },
            z_index: ZIndex::Global(15),
            transform: Transform::from_translation(cursor_spawn),
            ..default()
        },
        GameCursor {}
    ));
}

pub fn move_cursor(window: Query<&Window>, mut cursor: Query<&mut Style, With<GameCursor>>) {
    let window: &Window = window.single();
    if let Some(position) = window.cursor_position() {
        if let Ok(mut style) = cursor.get_single_mut() {
            style.position.left = Val::Px(position.x - 2.0);
            style.position.bottom = Val::Px(position.y - 24.0);
        }
    }
}

pub fn update_icon(
    asset_server: Res<AssetServer>,
    input: Res<Input<MouseButton>>,
    mut cursor_query: Query<&mut UiImage, With<GameCursor>>
) {
    if let Ok(mut cursor) = cursor_query.get_single_mut() {
        if input.pressed(MouseButton::Left) || input.pressed(MouseButton::Right) {
            cursor.texture  = asset_server.load("cursors/click.png");
        } else {
            cursor.texture = asset_server.load("cursors/point.png");
        }
    }
}

pub fn despawn_cursor(mut commands: Commands, cursor_query: Query<Entity, With<GameCursor>>) {
    if let Ok(cursor) = cursor_query.get_single() {
        commands.entity(cursor).despawn();
    }
}