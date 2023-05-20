use bevy::prelude::*;

use crate::game::PauseState;

// pub fn pause_game(mut simulation_state_next_state: ResMut<NextState<PauseState>>) {
//     simulation_state_next_state.set(PauseState::Paused);
// }

// pub fn resume_game(mut simulation_state_next_state: ResMut<NextState<PauseState>>) {
//     simulation_state_next_state.set(PauseState::Running);
// }

pub fn toggle_pause(
    keyboard_input: Res<Input<KeyCode>>,
    pause_state: Res<State<PauseState>>,
    mut next_state: ResMut<NextState<PauseState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if pause_state.0 == PauseState::Running {
            next_state.set(PauseState::Paused);
        }
        if pause_state.0 == PauseState::Paused {
            next_state.set(PauseState::Running);
        }
    }
}
