use bevy::prelude::*;

#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
pub struct Video {
    full_screen: bool
}

impl Default for Video {
    fn default() -> Self {
        Video { full_screen: false }
    }
}

#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
pub struct Volume {
    sound: i32,
    music: i32
}

impl Default for Volume {
    fn default() -> Self {
        Volume { sound: 100, music: 100 }
    }
}