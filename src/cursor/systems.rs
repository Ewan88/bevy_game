use super::components::*;
use bevy::prelude::*;

pub fn setup_cursor_icons(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(Cursors {
        point: asset_server.load("cursors/point.png").into(),
        click: asset_server.load("cursors/click.png").into(),
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
    //let cursor: Handle<Image> = cursors.point.clone().into();

    commands.spawn((
        ImageBundle {
            image: cursors.point.clone().into(),
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
        GameCursor {},
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
    input: Res<Input<MouseButton>>,
    mut cursor_query: Query<&mut UiImage, With<GameCursor>>,
    cursors: Res<Cursors>,
) {
    if let Ok(mut cursor) = cursor_query.get_single_mut() {
        if input.pressed(MouseButton::Left) || input.pressed(MouseButton::Right) {
            cursor.texture = cursors.click.clone();
        } else {
            cursor.texture = cursors.point.clone();
        }
    }
}

#[allow(dead_code)]
pub fn despawn_cursor(mut commands: Commands, cursor_query: Query<Entity, With<GameCursor>>) {
    if let Ok(cursor) = cursor_query.get_single() {
        commands.entity(cursor).despawn();
    }
}

// pub fn spawn_ui_cursor(
//     mut windows: Query<&mut Window>,
//     mut commands: Commands,
//     cursors: Res<Cursors>,
// ) {
//     let mut window: Mut<Window> = windows.single_mut();
//     window.cursor.visible = false;
//     let cursor_image: Handle<Image> = cursors.point.clone().into();

//     commands.spawn(UiCursorBundle {
//         cursor: UiCursor {
//             image: cursor_image.clone(),
//             size: vec2(288.0, 288.0),
//         },
//         ..default()
//     });
// }

// pub fn extract_ui_cursor(
//     mut extracted_uinodes: ResMut<ExtractedUiNodes>,
//     cursor_query: Extract<Query<(&GlobalTransform, &UiCursor)>>,
// ) {
//     for (transform, cursor) in cursor_query.iter() {
//         extracted_uinodes.uinodes.push(ExtractedUiNode {
//             stack_index: usize::MAX,
//             transform: transform.compute_matrix(),
//             color: Color::WHITE,
//             rect: Rect {
//                 min: Vec2::ZERO,
//                 max: cursor.size,
//             },
//             image: cursor.image.clone_weak(),
//             atlas_size: None,
//             clip: None,
//             flip_x: false,
//             flip_y: false,
//         });
//     }
// }
