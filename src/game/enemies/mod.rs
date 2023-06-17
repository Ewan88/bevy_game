mod components;
mod systems;
use self::systems::*;
use crate::prelude::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_enemy_icons)
            .add_system(spawn_enemy.in_schedule(OnEnter(GameState::Game)));
    }
}
