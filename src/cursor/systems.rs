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
    cursors: Res<Cursors>
) {
    let mut window: Mut<Window> = windows.single_mut();
    window.cursor.visible = false;
    let cursor_spawn: Vec3 = Vec3::ZERO;

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
    cursors: Res<Cursors>
) {
    if let Ok(mut cursor) = cursor_query.get_single_mut() {
        if input.pressed(MouseButton::Left) || input.pressed(MouseButton::Right) {
            cursor.texture = cursors.click.clone();
        } else {
            cursor.texture = cursors.point.clone();
        }
    }
}

pub fn despawn_cursor(mut commands: Commands, cursor_query: Query<Entity, With<GameCursor>>) {
    if let Ok(cursor) = cursor_query.get_single() {
        commands.entity(cursor).despawn();
    }
}

// use bevy::prelude::*;
// use bevy::render::camera::RenderTarget;

// // ANCHOR: example
// /// Used to help identify our main camera
// #[derive(Component)]
// struct MainCamera;

// fn setup(mut commands: Commands) {
//     commands.spawn((Camera2dBundle::default(), MainCamera));
// }

// fn my_cursor_system(
//     // need to get window dimensions
//     windows: Res<Windows>,
//     // query to get camera transform
//     camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
// ) {
//     // get the camera info and transform
//     // assuming there is exactly one main camera entity, so query::single() is OK
//     let (camera, camera_transform) = camera_q.single();

//     // get the window that the camera is displaying to (or the primary window)
//     let window = if let RenderTarget::Window(id) = camera.target {
//         windows.get(id).unwrap()
//     } else {
//         windows.get_primary().unwrap()
//     };

//     // check if the cursor is inside the window and get its position
//     // then, ask bevy to convert into world coordinates, and truncate to discard Z
//     if let Some(world_position) = window.cursor_position()
//         .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
//         .map(|ray| ray.origin.truncate())
//     {
//         eprintln!("World coords: {}/{}", world_position.x, world_position.y);
//     }
// }
// // ANCHOR_END: example

// fn main() {
//     App::new()
//         .add_plugins(DefaultPlugins)
//         .add_startup_system(setup)
//         .add_system(my_cursor_system)
//         .run();
// }
