// define height and width as screen size
// we can change this later by passing in a map size
pub const NUM_TILES: usize = 0;

#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    pub tiles: Vec<TileType>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }
}
