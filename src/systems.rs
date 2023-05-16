use bevy::prelude::*;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn transition_to_game_state() {

}

pub fn transition_to_main_menu_state() {

}

pub fn exit_game() {
    
}