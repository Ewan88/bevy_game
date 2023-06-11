use bevy::prelude::*;

use super::components::*;

#[derive(Resource)]
pub struct MapBuilder {
    pub map: Map,
    pub rooms: Rect,
    pub player_start: Vec2,
}

impl MapBuilder {
    pub fn new(width: i32, height: i32) -> Self {
        let mut mb: MapBuilder = Self {
            map: Map::new(),
            rooms: Rect::new(0., 0., width as f32, height as f32),
            player_start: Vec2::new(0.0, 0.0),
        };
        mb.fill(TileType::Floor);
        mb
    }

    pub fn fill(&mut self, tile: TileType) {
        self.map.tiles.iter_mut().for_each(|t| *t = tile);
    }
}
