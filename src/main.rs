mod main_menu;
mod systems;
mod cursor;
mod settings;

use crate::cursor::*;

use main_menu::MainMenuPlugin;
use systems::*;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<GameState>()
        .add_plugin(MainMenuPlugin)
        //.add_plugin(GamePlugin)
        .add_startup_system(setup_camera)
        //.insert_resource(Video::default())
        //.insert_resource(Volume::default())
        .add_startup_system(setup_cursor)
        .add_system(move_cursor)
        .add_system(update_icon)
        .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    Game,
    Settings,
}

// pub mod menu;
// pub mod game;

// use bevy::prelude::*;



// #[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
// pub enum GameState {
//     #[default]
//     // TODO: add loading screen
//     Menu,
//     Game
// }

// fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
//     for entity in &to_despawn {
//         commands.entity(entity).despawn_recursive();
//     }
// }
