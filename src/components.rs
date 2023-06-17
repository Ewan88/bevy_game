use crate::prelude::*;

// these aren't really components, but they're used in a similar way

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    Game,
    Settings,
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct MovementSystemSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct ConfinementSystemSet;

use bevy::utils::HashSet;

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct DespawnSet(pub HashSet<Entity>);

#[allow(dead_code)]
impl DespawnSet {
    pub fn apply(mut commands: Commands, mut despawn: ResMut<Self>) {
        for entity in despawn.0.drain() {
            commands.entity(entity).despawn_recursive();
        }
    }
}
