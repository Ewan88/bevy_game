use bevy::prelude::*;

#[derive(Component)]
pub struct GameCursor();

#[derive(Resource)]
pub struct Cursors {
    pub point: Handle<Image>,
    pub click: Handle<Image>,
}
