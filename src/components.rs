use crate::prelude::*;

// these aren't really components, but they're used in a similar way

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    Game,
    Settings,
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct EntityMovementSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct EntityConfinementSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct CameraMovementSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct CameraConfinementSet;
