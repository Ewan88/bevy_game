use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::components::*;

pub fn build_map() {}

// call spawn map to draw
pub fn spawn_map(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
}
