mod player;
mod systems;
mod enemies;
mod ui;

use crate::GameState;
use ::bevy::prelude::*;
use player::PlayerPlugin;
use systems::*;


pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<PauseState>()
            .add_plugin(PlayerPlugin)
            .add_system(toggle_pause.run_if(in_state(GameState::Game)));
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum PauseState {
    #[default]
    Running,
    Paused,
}
