use super::components::*;
use crate::prelude::*;

pub fn spawn_map(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // total tiles = 80 * 50 = 4000
    for y in 0..Y_TILES as i32 {
        for x in 0..X_TILES as i32 {
            let transform = Transform::from_xyz(
                x as f32 * TILE_SIZE,
                y as f32 * TILE_SIZE,
                0.0,
            );
            commands.spawn((
                PbrBundle {
                    transform,
                    mesh: meshes
                        .add(shape::Plane::from_size(2.0).into()),
                    material: materials.add(Color::rgb(0.0, 1.0, 0.0).into()),
                    ..default()
                },
                GameMap {},
            ));
        }
    }
}
