use bevy::{app::AppExit, prelude::*};

use super::{despawn_screen, GameState, FONT_PATH, TEXT_COLOR};

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const HOVERED_PRESSED_BUTTON: Color = Color::rgb(0.25, 0.65, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<MenuState>()
            .add_system(menu_setup.in_schedule(OnEnter(GameState::Menu)))
            .add_systems((
                main_menu_setup.in_schedule(OnEnter(MenuState::Main)),
                despawn_screen::<OnMainMenuScreen>.in_schedule(OnExit(MenuState::Main)),
            ))
            .add_systems((
                settings_menu_setup.in_schedule(OnEnter(MenuState::Settings)),
                despawn_screen::<OnSettingsMenuScreen>.in_schedule(OnExit(MenuState::Settings)),
            ))
            .add_systems((menu_action, button_system).in_set(OnUpdate(GameState::Menu)));
    }
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum MenuState {
    Main,
    Settings,
    #[default]
    Disabled,
}

#[derive(Component)]
struct OnMainMenuScreen;

#[derive(Component)]
struct OnSettingsMenuScreen;

#[derive(Component)]
struct SelectedOption;

#[derive(Component)]
enum MenuButtonAction {
    Play,
    Settings,
    BackToMainMenu,
    Quit,
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, selected) in &mut interaction_query {
        *color = match (*interaction, selected) {
            (Interaction::Clicked, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
            (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
            (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
            (Interaction::None, None) => NORMAL_BUTTON.into(),
        }
    }
}

#[allow(dead_code)]
fn setting_button<T: Resource + Component + PartialEq + Copy>(
    interaction_query: Query<(&Interaction, &T, Entity), (Changed<Interaction>, With<Button>)>,
    mut selected_query: Query<(Entity, &mut BackgroundColor), With<SelectedOption>>,
    mut commands: Commands,
    mut setting: ResMut<T>,
) {
    for (interaction, button_setting, entity) in &interaction_query {
        if *interaction == Interaction::Clicked && *setting != *button_setting {
            let (previous_button, mut previous_color) = selected_query.single_mut();
            *previous_color = NORMAL_BUTTON.into();
            commands.entity(previous_button).remove::<SelectedOption>();
            commands.entity(entity).insert(SelectedOption);
            *setting = *button_setting;
        }
    }
}

fn menu_setup(mut menu_state: ResMut<NextState<MenuState>>) {
    menu_state.set(MenuState::Main);
}

fn main_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font: Handle<Font> = asset_server.load(FONT_PATH);

    let button_style: Style = Style {
        size: Size::new(Val::Px(250.0), Val::Px(65.0)),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let button_text_style = TextStyle {
        font: font.clone(),
        font_size: 40.0,
        color: TEXT_COLOR,
    };

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            OnMainMenuScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::CRIMSON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(
                        TextBundle::from_section(
                            "Bevy Game", // global?
                            TextStyle {
                                font: font.clone(),
                                font_size: 80.0,
                                color: TEXT_COLOR,
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(50.0)),
                            ..default()
                        }),
                    );

                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            MenuButtonAction::Play,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "New Game",
                                button_text_style.clone(),
                            ));
                        });
                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            MenuButtonAction::Settings,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Settings",
                                button_text_style.clone(),
                            ));
                        });
                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style,
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            MenuButtonAction::Quit,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section("Quit", button_text_style));
                        });
                });
        });
}

fn settings_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let button_style = Style {
        size: Size::new(Val::Px(200.0), Val::Px(65.0)),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let button_text_style = TextStyle {
        font: asset_server.load(FONT_PATH),
        font_size: 40.0,
        color: TEXT_COLOR,
    };

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            OnSettingsMenuScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::CRIMSON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            MenuButtonAction::BackToMainMenu,
                        ))
                        .with_children(|parent| {
                            parent
                                .spawn(TextBundle::from_section("Back", button_text_style.clone()));
                        });
                });
        });
}

fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Clicked {
            match menu_button_action {
                MenuButtonAction::Quit => app_exit_events.send(AppExit),
                MenuButtonAction::Play => {
                    game_state.set(GameState::Game);
                    menu_state.set(MenuState::Disabled);
                }
                MenuButtonAction::Settings => {
                    menu_state.set(MenuState::Settings);
                }
                MenuButtonAction::BackToMainMenu => {
                    menu_state.set(MenuState::Main);
                }
            }
        }
    }
}
