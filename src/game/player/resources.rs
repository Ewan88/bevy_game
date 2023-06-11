use crate::prelude::*;
use bevy::utils::HashSet;

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct DespawnSet(pub HashSet<Entity>);

impl DespawnSet {
    pub fn apply(mut commands: Commands, mut despawn: ResMut<Self>) {
        for entity in despawn.0.drain() {
            commands.entity(entity).despawn_recursive();
        }
    }
}
