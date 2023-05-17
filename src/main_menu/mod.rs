use bevy::prelude::*;

mod components;
mod styles;
mod systems;

use systems::interactions::*;
use systems::layout::*;

use crate::GameState;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_main_menu.in_schedule(OnEnter(GameState::MainMenu)))
            .add_systems(
                (
                    interact_with_new_game_button,
                    interact_with_quit_button,
                    interact_with_settings_button,
                )
                .in_set(OnUpdate(GameState::MainMenu)),
            )
            .add_system(despawn_main_menu.in_schedule(OnExit(GameState::MainMenu)));
    }
}

// #[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
// enum MenuState {
//     Main,
//     Settings,
//     #[default]
//     Disabled,
// }

// pub mod settings;

// use bevy::{app::AppExit, prelude::*};

// //use self::settings::{settings_menu_setup};

// use super::{despawn_screen, GameState, FONT_PATH, TEXT_COLOR};

// fn menu_setup(mut menu_state: ResMut<NextState<MenuState>>) {
//     menu_state.set(MenuState::Main);
// }

// fn main_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
//     
// }