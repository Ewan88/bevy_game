mod components;
mod resources;
mod systems;
use crate::prelude::*;
use systems::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(
            Update,
            MovementSystemSet.before(ConfinementSystemSet),
        )
        .add_systems(OnEnter(GameState::Game), spawn_player)
        .add_systems(
            Update,
            (
                move_player.in_set(MovementSystemSet),
                update_player.in_set(MovementSystemSet),
                confine_player_movement.in_set(ConfinementSystemSet),
            )
                .in_set(GameState::Game)
                .in_set(PauseState::Running),
        )
        .add_systems(OnExit(GameState::Game), despawn_player);
    }
}
