mod camera;
mod components;
mod cursor;
mod game;
mod main_menu;
mod settings;

mod prelude {
    pub use bevy::prelude::*;
    pub use bevy::window::*;
    pub const SCREEN_WIDTH: f32 = 80.0 * 32.0;
    pub const SCREEN_HEIGHT: f32 = 50.0 * 32.0;
    pub const DISPLAY_WIDTH: f32 = SCREEN_WIDTH / 2.0;
    pub const DISPLAY_HEIGHT: f32 = SCREEN_HEIGHT / 2.0;
    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::cursor::*;
    pub use crate::game::*;
    pub use crate::main_menu::*;
    pub use crate::settings::*;
    pub use crate::systems::*;
}

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
        .add_plugin(GamePlugin)
        .add_startup_system(setup_camera)
        .run();
}
