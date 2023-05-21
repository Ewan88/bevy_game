use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub destination: Option<Vec2>
}

#[derive(Component)]
pub struct MoveIcon();
