use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub destination: Option<Vec2>
}
