mod components;
mod systems;
use crate::prelude::*;
use systems::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_tile_icons)
            .add_system(spawn_map.in_schedule(OnEnter(GameState::Game)));
    }
}
