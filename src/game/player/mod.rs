mod components;
mod resources;
mod systems;
use systems::*;

use self::resources::DespawnSet;
use crate::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(EntityMovementSet.before(EntityConfinementSet))
            .init_resource::<DespawnSet>()
            .add_system(spawn_player.in_schedule(OnEnter(GameState::Game)))
            .add_systems(
                (
                    move_player.in_set(EntityMovementSet),
                    update_player.in_set(EntityMovementSet),
                    confine_player_movement.in_set(EntityConfinementSet),
                    DespawnSet::apply,
                )
                    .in_set(OnUpdate(GameState::Game))
                    .in_set(OnUpdate(PauseState::Running)),
            )
            .add_system(despawn_player.in_schedule(OnExit(GameState::Game)));
    }
}
