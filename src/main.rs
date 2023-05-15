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
        .add_startup_system(setup_cursor)
        .add_state::<GameState>()
        .add_plugin(menu::MenuPlugin)
        .add_plugin(game::GamePlugin)
        .add_system(move_cursor)
        .add_system(update_icon)
        .run();
}

#[derive(Component)]
struct GameCursor {}

fn setup_cursor(
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

fn move_cursor(window: Query<&Window>, mut cursor: Query<&mut Style, With<GameCursor>>) {
    let window: &Window = window.single();
    if let Some(position) = window.cursor_position() {
        let mut style: Mut<Style> = cursor.single_mut();
        style.position.left = Val::Px(position.x - 2.0);
        style.position.bottom = Val::Px(position.y - 24.0);
    }
}

fn update_icon(
    asset_server: Res<AssetServer>,
    input: Res<Input<MouseButton>>,
    mut cursor_query: Query<&mut UiImage, With<GameCursor>>
) {
    let mut cursor: Mut<UiImage> = cursor_query.single_mut();
    if input.pressed(MouseButton::Left) || input.pressed(MouseButton::Right) {
        cursor.texture  = asset_server.load("cursors/click.png");
    } else {
        cursor.texture = asset_server.load("cursors/point.png");
    }
}
