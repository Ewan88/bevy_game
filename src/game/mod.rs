// pub struct GamePlugin;

// impl Plugin for GamePlugin {
//     fn build(&self, app: &mut App) {
//         app.add_systems((
//             game_setup.in_schedule(OnEnter(GameState::Game)),
//             game.in_set(OnUpdate(GameState::Game)),
//             despawn_screen::<OnGameScreen>.in_schedule(OnExit(GameState::Game)),
//         ));
//     }
// }


// use bevy::prelude::*;
// use crate::Volume;

// use super::{despawn_screen, GameState, FONT_PATH, TEXT_COLOR};

// #[derive(Component)]
// struct OnGameScreen;

// fn game_setup(mut commands: Commands, asset_server: Res<AssetServer>, volume: Res<Volume>) {
//     let font = asset_server.load(FONT_PATH);

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
//             OnGameScreen,
//         ))
//         .with_children(|parent| {
//             parent
//                 .spawn(NodeBundle {
//                     style: Style {
//                         flex_direction: FlexDirection::Column,
//                         align_items: AlignItems::Center,
//                         ..default()
//                     },
//                     background_color: Color::BLACK.into(),
//                     ..default()
//                 })
//                 .with_children(|parent| {
//                     parent.spawn(
//                         TextBundle::from_section(
//                             "Returning to menu...",
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
//                     parent.spawn(
//                         TextBundle::from_section(
//                             format!("volume: {:?}", *volume),
//                             TextStyle {
//                                 font: font.clone(),
//                                 font_size: 60.0,
//                                 color: Color::GREEN,
//                             },
//                         )
//                         .with_style(Style {
//                             margin: UiRect::all(Val::Px(50.0)),
//                             ..default()
//                         }),
//                     );
//                 });
//         });
//     commands.insert_resource(GameTimer(Timer::from_seconds(5.0, TimerMode::Once)));
// }

// fn game(
//     time: Res<Time>,
//     mut game_state: ResMut<NextState<GameState>>,
//     mut timer: ResMut<GameTimer>,
// ) {
//     if timer.tick(time.delta()).finished() {
//         game_state.set(GameState::Menu);
//     }
// }
