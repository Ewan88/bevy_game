use super::components::*;
use crate::prelude::*;

pub fn spawn_map(mut commands: Commands, tiles: Res<TileTypes>) {
    for y in 0..Y_TILES as i32 {
        for x in 0..X_TILES as i32 {
            let transform = Transform::from_xyz(
                x as f32 * TILE_SIZE,
                y as f32 * TILE_SIZE,
                0.0,
            );
            commands.spawn((
                SpriteBundle {
                    transform,
                    texture: tiles.grass.clone(),
                    ..default()
                },
                RenderLayers::layer(0),
            ));
        }
    }
}

pub fn setup_tile_icons(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(TileTypes {
        grass: asset_server.load("sprites/terrain/grass.png"),
        wall: asset_server.load("sprites/terrain/wall.png"),
    });
}
