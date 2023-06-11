mod cursor;
mod game;
mod main_menu;
mod settings;
mod systems;

use crate::cursor::*;

use game::GamePlugin;
use main_menu::MainMenuPlugin;
use systems::*;

use bevy::{prelude::*, window::*};

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    Game,
    Settings,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Game!".to_string(),
                mode: WindowMode::Windowed,
                resolution: WindowResolution::new(1280.0, 700.0),
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
