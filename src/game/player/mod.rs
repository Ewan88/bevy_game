mod components;
mod resources;
mod systems;
use crate::GameState;
use bevy::prelude::*;
use systems::*;

use crate::game::player::resources::DespawnSet;

use super::PauseState;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct MovementSystemSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct ConfinementSystemSet;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(MovementSystemSet.before(ConfinementSystemSet))
            .init_resource::<DespawnSet>()
            .add_system(spawn_player.in_schedule(OnEnter(GameState::Game)))
            .add_systems(
                (
                    move_player.in_set(MovementSystemSet),
                    update_player.in_set(MovementSystemSet),
                    confine_player_movement.in_set(ConfinementSystemSet),
                    DespawnSet::apply,
                )
                    .in_set(OnUpdate(GameState::Game))
                    .in_set(OnUpdate(PauseState::Running)),
            )
            .add_system(despawn_player.in_schedule(OnExit(GameState::Game)));
    }
}
