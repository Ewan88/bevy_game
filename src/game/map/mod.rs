mod components;
mod systems;
use crate::prelude::*;
use systems::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_tile_icons)
            .add_systems(OnEnter(GameState::Game), spawn_map);
    }
}
