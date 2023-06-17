use super::components::*;
use crate::prelude::*;

const TILE_SIZE: f32 = 8.0;

pub fn spawn_map(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for z in 0..2 as i32 {
        for x in 0..2 as i32 {
            let transform = Transform::from_xyz(
                x as f32 * TILE_SIZE,
                0.0,
                z as f32 * TILE_SIZE,
            );
            commands.spawn((
                PbrBundle {
                    transform,
                    mesh: meshes.add(shape::Plane::from_size(TILE_SIZE).into()),
                    material: materials.add(Color::rgb(0.0, 1.0, 0.0).into()),
                    ..default()
                },
                GameMap {},
            ));
        }
    }
}
