use crate::prelude::*;

#[derive(Component)]
pub struct Enemy();

#[derive(Resource)]
pub struct EnemyTypes {
    pub enemy: Handle<Image>,
}
