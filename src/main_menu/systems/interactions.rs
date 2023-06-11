use bevy::app::AppExit;
use bevy::prelude::*;

use crate::main_menu::components::*;
use crate::main_menu::styles::{HOVERED_BUTTON, NORMAL_BUTTON, PRESSED_BUTTON};
use crate::GameState;

pub fn interact_with_new_game_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<NewGameButton>),
    >,
    mut game_state: ResMut<NextState<GameState>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    if let Ok((interaction, mut background_color)) =
        button_query.get_single_mut()
    {
        match *interaction {
            Interaction::Clicked => {
                *background_color = PRESSED_BUTTON.into();
                play_sound_menu_click(asset_server, audio);
                game_state.set(GameState::Game);
            }
            Interaction::Hovered => {
                *background_color = HOVERED_BUTTON.into();
                play_sound_menu_hover(asset_server, audio);
            }
            Interaction::None => {
                *background_color = NORMAL_BUTTON.into();
            }
        }
    }
}

pub fn interact_with_settings_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<SettingsButton>),
    >,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    if let Ok((interaction, mut background_color)) =
        button_query.get_single_mut()
    {
        match *interaction {
            Interaction::Clicked => {
                *background_color = PRESSED_BUTTON.into();
                play_sound_menu_click(asset_server, audio);
                // spawn settings window
            }
            Interaction::Hovered => {
                *background_color = HOVERED_BUTTON.into();
                play_sound_menu_hover(asset_server, audio);
            }
            Interaction::None => {
                *background_color = NORMAL_BUTTON.into();
            }
        }
    }
}

pub fn interact_with_quit_button(
    mut event_writer: EventWriter<AppExit>,
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<QuitButton>),
    >,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    if let Ok((interaction, mut background_color)) =
        button_query.get_single_mut()
    {
        match *interaction {
            Interaction::Clicked => {
                *background_color = PRESSED_BUTTON.into();
                play_sound_menu_click(asset_server, audio);
                event_writer.send(AppExit);
            }
            Interaction::Hovered => {
                *background_color = HOVERED_BUTTON.into();
                play_sound_menu_hover(asset_server, audio);
            }
            Interaction::None => {
                *background_color = NORMAL_BUTTON.into();
            }
        }
    }
}

fn play_sound_menu_hover(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    audio.play(asset_server.load("sounds/menu/blipC3.ogg"));
}

fn play_sound_menu_click(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    audio.play(asset_server.load("sounds/menu/blipC2.ogg"));
}
