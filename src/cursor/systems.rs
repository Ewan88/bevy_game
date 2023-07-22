use super::components::*;
use crate::prelude::*;

pub fn setup_cursor_icons(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(Cursors {
        point: asset_server.load("cursors/point.png"),
        click: asset_server.load("cursors/click.png"),
    });
}

pub fn spawn_menu_cursor(
    mut windows: Query<&mut Window>,
    mut commands: Commands,
    cursors: Res<Cursors>,
) {
    let mut window: Mut<Window> = windows.single_mut();
    window.cursor.visible = false;
    let cursor_spawn: Vec3 = Vec3::ZERO;

    commands.spawn((
        ImageBundle {
            image: cursors.point.clone().into(),
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(24.),
                right: Val::Px(24.),
                ..default()
            },
            z_index: ZIndex::Global(999),
            transform: Transform::from_translation(cursor_spawn),
            ..default()
        },
        GameCursor {},
        RenderLayers::all(),
    ));
}

pub fn move_cursor(
    window: Query<&Window>,
    mut cursor: Query<&mut Style, With<GameCursor>>,
) {
    let window: &Window = window.single();
    if let Some(position) = window.cursor_position() {
        if let Ok(mut style) = cursor.get_single_mut() {
            style.left = Val::Px(position.x - 2.0);
            style.bottom = Val::Px(position.y - 24.0);
        }
    }
}

pub fn update_icon(
    input: Res<Input<MouseButton>>,
    mut cursor_query: Query<&mut UiImage, With<GameCursor>>,
    cursors: Res<Cursors>,
) {
    // need to query for player location
    // need to query for enemy location
    if let Ok(mut cursor) = cursor_query.get_single_mut() {
        if input.pressed(MouseButton::Left) || input.pressed(MouseButton::Right)
        {
            cursor.texture = cursors.click.clone();
        } else {
            cursor.texture = cursors.point.clone();
        }
    }
}

#[allow(dead_code)]
pub fn despawn_cursor(
    mut commands: Commands,
    cursor_query: Query<Entity, With<GameCursor>>,
) {
    if let Ok(cursor) = cursor_query.get_single() {
        commands.entity(cursor).despawn();
    }
}
