use crate::prelude::*;

mod components;
mod styles;
mod systems;

use systems::interactions::*;
use systems::layout::*;

use crate::GameState;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), spawn_main_menu)
            .add_systems(
                Update,
                (
                    interact_with_new_game_button,
                    interact_with_quit_button,
                    interact_with_settings_button,
                ),
            )
            .add_systems(OnExit(GameState::MainMenu), despawn_main_menu);
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
