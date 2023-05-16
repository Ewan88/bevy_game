pub mod systems;
pub mod components;

// use bevy::prelude::*;
// use crate::game_state::Volume;
// use super::{
//     SelectedOption, OnSettingsMenuScreen, MenuButtonAction, NORMAL_BUTTON, FONT_PATH, TEXT_COLOR
// };

// pub fn setting_button<T: Resource + Component + PartialEq + Copy>(
//     interaction_query: Query<(&Interaction, &T, Entity), (Changed<Interaction>, With<Button>)>,
//     mut selected_query: Query<(Entity, &mut BackgroundColor), With<SelectedOption>>,
//     mut commands: Commands,
//     mut setting: ResMut<T>,
// ) {
//     for (interaction, button_setting, entity) in &interaction_query {
//         if *interaction == Interaction::Clicked && *setting != *button_setting {
//             let (previous_button, mut previous_color)
//                 = selected_query.single_mut();
//             *previous_color = NORMAL_BUTTON.into();
//             commands.entity(previous_button).remove::<SelectedOption>();
//             commands.entity(entity).insert(SelectedOption);
//             *setting = *button_setting;
//         }
//     }
// }

// pub fn settings_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
//     let button_style = Style {
//         size: Size::new(Val::Px(200.0), Val::Px(65.0)),
//         margin: UiRect::all(Val::Px(20.0)),
//         justify_content: JustifyContent::Center,
//         align_items: AlignItems::Center,
//         ..default()
//     };

//     let button_text_style = TextStyle {
//         font: asset_server.load(FONT_PATH),
//         font_size: 40.0,
//         color: TEXT_COLOR,
//     };

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
//             OnSettingsMenuScreen,
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
//                     parent
//                         .spawn((
//                             ButtonBundle {
//                                 style: button_style.clone(),
//                                 background_color: NORMAL_BUTTON.into(),
//                                 ..default()
//                             },
//                             MenuButtonAction::BackToMainMenu,
//                         ))
//                         .with_children(|parent| {
//                             parent
//                                 .spawn(
//                                     TextBundle::from_section(
//                                         "Back", button_text_style.clone()
//                                     )
//                                 );
//                         });
//                 });
//         });
// }

// fn sound_settings_menu_setup(mut commands: Commands, volume: Res<Volume>) {
//     let button_style = Style {
//         size: Size::new(Val::Px(200.0), Val::Px(65.0)),
//         margin: UiRect::all(Val::Px(20.0)),
//         justify_content: JustifyContent::Center,
//         align_items: AlignItems::Center,
//         ..default()
//     };
//     let button_text_style = TextStyle {
//         font_size: 40.0,
//         color: TEXT_COLOR,
//         ..default()
//     };

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
//             OnSoundSettingsMenuScreen,
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
//                     parent
//                         .spawn(NodeBundle {
//                             style: Style {
//                                 align_items: AlignItems::Center,
//                                 ..default()
//                             },
//                             background_color: Color::CRIMSON.into(),
//                             ..default()
//                         })
//                         .with_children(|parent| {
//                             parent.spawn(TextBundle::from_section(
//                                 "Volume",
//                                 button_text_style.clone(),
//                             ));
//                             for volume_setting in [0, 1, 2, 3, 4, 5, 6, 7, 8, 9] {
//                                 let mut entity = parent.spawn(ButtonBundle {
//                                     style: Style {
//                                         size: Size::new(Val::Px(30.0), Val::Px(65.0)),
//                                         ..button_style.clone()
//                                     },
//                                     background_color: NORMAL_BUTTON.into(),
//                                     ..default()
//                                 });
//                                 entity.insert(Volume(volume_setting));
//                                 if *volume == Volume(volume_setting) {
//                                     entity.insert(SelectedOption);
//                                 }
//                             }
//                         });
//                     parent
//                         .spawn((
//                             ButtonBundle {
//                                 style: button_style,
//                                 background_color: NORMAL_BUTTON.into(),
//                                 ..default()
//                             },
//                             MenuButtonAction::BackToSettings,
//                         ))
//                         .with_children(|parent| {
//                             parent.spawn(TextBundle::from_section("Back", button_text_style));
//                         });
//                 });
//         });
// }