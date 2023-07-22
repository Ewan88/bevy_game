mod camera;
mod components;
mod cursor;
mod game;
mod main_menu;
mod settings;

mod prelude {
    pub use bevy::prelude::*;
    pub use bevy::render::view::RenderLayers;

    pub const TILE_SIZE: f32 = 32.0;
    pub const X_TILES: f32 = 80.0;
    pub const Y_TILES: f32 = 50.0;
    // map size in pixels is 2560x1600
    pub const MAP_WIDTH: f32 = X_TILES * TILE_SIZE;
    pub const MAP_HEIGHT: f32 = Y_TILES * TILE_SIZE;
    // display size in pixels is 1280x800
    pub const DISPLAY_WIDTH: f32 = MAP_WIDTH / 2.0;
    pub const DISPLAY_HEIGHT: f32 = MAP_HEIGHT / 2.0;

    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::cursor::*;
    pub use crate::game::*;
    pub use crate::main_menu::*;
    pub use crate::settings::*;
}

use bevy::window::*;
use prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
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
            }),
            CursorPlugin,
            MainMenuPlugin,
            CameraPlugin,
            GamePlugin,
        ))
        .add_state::<GameState>()
        .init_resource::<DespawnSet>()
        .add_systems(Update, DespawnSet::apply)
        .run();
}
