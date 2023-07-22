mod components;
mod systems;
use self::systems::*;
use crate::prelude::*;

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_cursor_icons)
            .add_systems(OnEnter(GameState::MainMenu), spawn_menu_cursor)
            .add_systems(Update, (update_icon, move_cursor));
    }
}
