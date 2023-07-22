mod components;
mod systems;
use self::systems::*;
use crate::prelude::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_enemy_icons)
            .add_systems(OnEnter(GameState::Game), spawn_enemy);
    }
}
