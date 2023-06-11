use bevy::prelude::*;
use bevy::window::PrimaryWindow;

#[allow(dead_code)]
pub fn build_map() {}

#[allow(dead_code, unused_variables, unused_mut)]
pub fn spawn_map(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
}
