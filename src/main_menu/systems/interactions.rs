use crate::main_menu::components::*;
use crate::main_menu::styles::*;
use crate::prelude::*;
use bevy::app::*;
use bevy::audio::Volume;

pub fn interact_with_new_game_button(
    commands: Commands,
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<NewGameButton>),
    >,
    mut game_state: ResMut<NextState<GameState>>,
    asset_server: Res<AssetServer>,
) {
    if let Ok((interaction, mut background_color)) =
        button_query.get_single_mut()
    {
        match *interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON.into();
                play_sound_menu_click(commands, asset_server);
                game_state.set(GameState::Game);
            }
            Interaction::Hovered => {
                *background_color = HOVERED_BUTTON.into();
                play_sound_menu_hover(commands, asset_server);
            }
            Interaction::None => {
                *background_color = NORMAL_BUTTON.into();
            }
        }
    }
}

pub fn interact_with_settings_button(
    commands: Commands,
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<SettingsButton>),
    >,
    asset_server: Res<AssetServer>,
) {
    if let Ok((interaction, mut background_color)) =
        button_query.get_single_mut()
    {
        match *interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON.into();
                play_sound_menu_click(commands, asset_server);
                // spawn settings window
            }
            Interaction::Hovered => {
                *background_color = HOVERED_BUTTON.into();
                play_sound_menu_hover(commands, asset_server);
            }
            Interaction::None => {
                *background_color = NORMAL_BUTTON.into();
            }
        }
    }
}

pub fn interact_with_quit_button(
    commands: Commands,
    mut event_writer: EventWriter<AppExit>,
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<QuitButton>),
    >,
    asset_server: Res<AssetServer>,
) {
    if let Ok((interaction, mut background_color)) =
        button_query.get_single_mut()
    {
        match *interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON.into();
                play_sound_menu_click(commands, asset_server);
                event_writer.send(AppExit);
            }
            Interaction::Hovered => {
                *background_color = HOVERED_BUTTON.into();
                play_sound_menu_hover(commands, asset_server);
            }
            Interaction::None => {
                *background_color = NORMAL_BUTTON.into();
            }
        }
    }
}

fn play_sound_menu_hover(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(AudioBundle {
        source: asset_server.load("sounds/menu/blipC3.ogg"),
        settings: PlaybackSettings::ONCE.with_volume(Volume::new_relative(1.)),
    });
}

fn play_sound_menu_click(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(AudioBundle {
        source: asset_server.load("sounds/menu/blipC2.ogg"),
        settings: PlaybackSettings::ONCE.with_volume(Volume::new_relative(1.)),
    });
}
