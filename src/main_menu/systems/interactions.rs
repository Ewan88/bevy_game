pub fn interact_with_new_game_button() {

}

pub fn interact_with_settings_button() {

}

pub fn interact_with_quit_button() {

}

// pub fn button_system(
//     mut interaction_query: Query<
//         (&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
//         (Changed<Interaction>, With<Button>),
//     >,
//     asset_server: Res<AssetServer>,
//     audio: Res<Audio>,
// ) {
//     for (interaction, mut color, selected) in &mut interaction_query {
//         *color = match (*interaction, selected) {
//             (Interaction::Clicked, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
//             (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
//             (Interaction::Hovered, None) => {
//                 audio.play(asset_server.load("sounds/menu/blip.ogg"));
//                 HOVERED_BUTTON.into()
//             }
//             (Interaction::None, None) => NORMAL_BUTTON.into(),
//         };
//     }
// }

// pub fn menu_action(
//     interaction_query: Query<
//         (&Interaction, &MenuButtonAction),
//         (Changed<Interaction>, With<Button>),
//     >,
//     mut app_exit_events: EventWriter<AppExit>,
//     mut menu_state: ResMut<NextState<MenuState>>,
//     mut game_state: ResMut<NextState<GameState>>,
// ) {
//     for (interaction, menu_button_action) in &interaction_query {
//         if *interaction == Interaction::Clicked {
//             match menu_button_action {
//                 MenuButtonAction::Quit => app_exit_events.send(AppExit),
//                 MenuButtonAction::Play => {
//                     game_state.set(GameState::Game);
//                     menu_state.set(MenuState::Disabled);
//                 }
//                 MenuButtonAction::Settings => {
//                     menu_state.set(MenuState::Settings);
//                 }
//                 MenuButtonAction::BackToMainMenu => {
//                     menu_state.set(MenuState::Main);
//                 }
//             }
//         }
//     }
// }