use crate::prelude::*;

#[derive(Component)]
pub struct Player {
    pub destination: Option<Vec3>,
}

#[derive(Component, PartialEq, Eq, Hash)]
pub struct MoveIcon();
