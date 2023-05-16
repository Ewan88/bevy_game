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

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum MenuState {
    Main,
    Settings,
    #[default]
    Disabled,
}

// pub mod settings;

// use bevy::{app::AppExit, prelude::*};

// //use self::settings::{settings_menu_setup};

// use super::{despawn_screen, GameState, FONT_PATH, TEXT_COLOR};

// fn menu_setup(mut menu_state: ResMut<NextState<MenuState>>) {
//     menu_state.set(MenuState::Main);
// }

// fn main_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
//     commands
//         .spawn((
//             NodeBundle {
//                 style: Style {
//                     size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
//                     align_items: AlignItems::Center,
//                     justify_content: JustifyContent::Center,
//                     ..default()
//                 },
//                 ..default()
//             },
//             OnMainMenuScreen,
//         ))
//         .with_children(|parent| {
//             parent
//                 .spawn(NodeBundle {
//                     style: Style {
//                         flex_direction: FlexDirection::Column,
//                         align_items: AlignItems::Center,
//                         ..default()
//                     },
//                     background_color: Color::CRIMSON.into(),
//                     ..default()
//                 })
//                 .with_children(|parent| {
//                     parent.spawn(
//                         TextBundle::from_section(
//                             "Bevy Game", // global?
//                             TextStyle {
//                                 font: font.clone(),
//                                 font_size: 80.0,
//                                 color: TEXT_COLOR,
//                             },
//                         )
//                         .with_style(Style {
//                             margin: UiRect::all(Val::Px(50.0)),
//                             ..default()
//                         }),
//                     );

//                     parent
//                         .spawn((
//                             ButtonBundle {
//                                 style: button_style.clone(),
//                                 background_color: NORMAL_BUTTON.into(),
//                                 ..default()
//                             },
//                             MenuButtonAction::Play,
//                         ))
//                         .with_children(|parent| {
//                             parent.spawn(TextBundle::from_section(
//                                 "New Game",
//                                 button_text_style.clone(),
//                             ));
//                         });
//                     parent
//                         .spawn((
//                             ButtonBundle {
//                                 style: button_style.clone(),
//                                 background_color: NORMAL_BUTTON.into(),
//                                 ..default()
//                             },
//                             MenuButtonAction::Settings,
//                         ))
//                         .with_children(|parent| {
//                             parent.spawn(TextBundle::from_section(
//                                 "Settings",
//                                 button_text_style.clone(),
//                             ));
//                         });
//                     parent
//                         .spawn((
//                             ButtonBundle {
//                                 style: button_style,
//                                 background_color: NORMAL_BUTTON.into(),
//                                 ..default()
//                             },
//                             MenuButtonAction::Quit,
//                         ))
//                         .with_children(|parent| {
//                             parent.spawn(TextBundle::from_section("Quit", button_text_style));
//                         });
//                 });
//         });
// }