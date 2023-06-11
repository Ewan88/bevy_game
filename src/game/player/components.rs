use bevy::{prelude::*, utils::HashSet};

#[derive(Component)]
pub struct Player {
    pub destination: Option<Vec2>,
}

#[derive(Component, PartialEq, Eq, Hash)]
pub struct MoveIcon();
