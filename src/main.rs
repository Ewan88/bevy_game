mod camera;
mod components;
mod cursor;
mod game;
mod main_menu;
mod settings;

mod prelude {
    pub use bevy::prelude::*;

    // pub const TILE_SIZE: f32 = 32.0;
    // in 3d space x is left/right, y is up/down, z is forward/backward
    // pub const X_TILES: f32 = 80.0;
    // pub const Y_TILES: f32 = 50.0;

    pub const DISPLAY_WIDTH: f32 = 1280.0;
    pub const DISPLAY_HEIGHT: f32 = 1024.0;

    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::cursor::components::*;
    pub use crate::cursor::*;
    pub use crate::game::*;
    pub use crate::main_menu::*;
    pub use crate::settings::*;
}

use bevy::window::*;
use prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Game!".to_string(),
                mode: WindowMode::Windowed,
                resolution: WindowResolution::new(
                    DISPLAY_WIDTH,
                    DISPLAY_HEIGHT,
                ),
                ..default()
            }),
            ..default()
        }))
        .add_state::<GameState>()
        .add_plugin(CursorPlugin)
        .add_plugin(MainMenuPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(GamePlugin)
        .run();
}
