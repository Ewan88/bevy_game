mod enemies;
mod map;
mod player;
mod systems;
mod ui;

use crate::prelude::*;
use systems::*;

use self::enemies::EnemyPlugin;
use self::map::*;
use self::player::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<PauseState>()
            .add_plugins((MapPlugin, PlayerPlugin, EnemyPlugin))
            .add_systems(
                Update,
                toggle_pause.run_if(in_state(GameState::Game)),
            );
    }
}

#[derive(
    SystemSet, States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default,
)]
pub enum PauseState {
    #[default]
    Running,
    Paused,
}
