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
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                mode: WindowMode::Windowed,
                //resolution: WindowResolution::new(800.0, 600.0),
                ..default()
            }),
            ..default()
        }))
        .add_state::<GameState>()
        .add_plugin(MainMenuPlugin)
        .add_plugin(GamePlugin)
        //.add_plugin(GamePlugin)
        .add_startup_system(setup_camera)
        //.insert_resource(Video::default())
        //.insert_resource(Volume::default())
        .add_startup_system(setup_cursor)
        .add_system(move_cursor)
        .add_system(update_icon)
        .run();
}
