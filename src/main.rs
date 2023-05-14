mod game_state;

use bevy::prelude::*;
use bevy::window::Cursor;
use game_state::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                cursor: Cursor::default(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(Video::default())
        .insert_resource(Volume::default())
        .add_startup_system(setup_camera)
        .add_startup_system(hide_cursor)
        .add_state::<GameState>()
        .add_plugin(menu::MenuPlugin)
        .add_plugin(game::GamePlugin)
        .add_system(set_cursor)
        .run();
}

fn hide_cursor(mut windows: Query<&mut Window>) {
    let mut window = windows.single_mut();
    window.cursor.visible = false;
}

fn set_cursor(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    input: Res<Input<MouseButton>>,
    window: Query<&Window>
) {
    let window: &Window = window.single();
    if let Some(position) = window.cursor_position() {
        let delta: Vec2 = position;
        println!("x: {}, y: {}", delta.x, delta.y);
        if input.pressed(MouseButton::Left) || input.pressed(MouseButton::Right) {
            commands.spawn(ImageBundle {
                image: asset_server.load("cursors/click.png").into(),
                style: Style {
                    position_type: PositionType::Absolute,
                    ..default()
                },
                z_index: ZIndex::Local(1),
                transform: Transform {
                    translation: Vec3 {
                        x: delta.x,
                        y: delta.y,
                        z: default(),
                    },
                    ..default()
                },
                ..default()
            });
        } else {
            commands.spawn(ImageBundle {
                image: asset_server.load("cursors/point.png").into(),
                style: Style {
                    position_type: PositionType::Absolute,
                    ..default()
                },
                z_index: ZIndex::Local(1),
                transform: Transform {
                    translation: Vec3 {
                        x: delta.x,
                        y: delta.y,
                        z: default(),
                    },
                    ..default()
                },
                ..default()
            });
        }
    }
}
