use crate::prelude::*;

#[derive(Component)]
pub struct Tile();

#[derive(Resource)]
pub struct TileTypes {
    pub grass: Handle<Image>,
    pub wall: Handle<Image>,
}
