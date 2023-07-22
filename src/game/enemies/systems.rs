use super::components::*;
use crate::prelude::*;

pub fn setup_enemy_icons(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(EnemyTypes {
        enemy: asset_server.load("sprites/enemy/enemy.png"),
    });
}

pub fn spawn_enemy(mut commands: Commands, enemies: Res<EnemyTypes>) {
    // spawn on top-right corner of map
    let transform = Transform::from_xyz(
        DISPLAY_WIDTH - (TILE_SIZE * 2.0),
        DISPLAY_HEIGHT - (TILE_SIZE * 2.0),
        1.0,
    );
    commands.spawn((
        SpriteBundle {
            transform,
            texture: enemies.enemy.clone(),
            ..default()
        },
        RenderLayers::layer(1),
        Enemy {},
    ));
}
