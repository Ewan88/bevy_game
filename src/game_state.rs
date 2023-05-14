pub mod menu;
pub mod game;

use bevy::prelude::*;

const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const FONT_PATH: &str = "fonts/RubikMaze-Regular.ttf"; 

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    // TODO: add loading screen
    Menu,
    Game
}

fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
pub struct Video {
    full_screen: bool
}

impl Default for Video {
    fn default() -> Self {
        Video { full_screen: false }
    }
}

#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
pub struct Volume {
    sound: i32,
    music: i32
}

impl Default for Volume {
    fn default() -> Self {
        Volume { sound: 100, music: 100 }
    }
}

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}