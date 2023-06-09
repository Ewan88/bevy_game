mod components;
mod systems;
use self::systems::*;
use crate::prelude::*;

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_cursor_icons)
            .add_system(
                spawn_menu_cursor.in_schedule(OnEnter(GameState::MainMenu)),
            )
            .add_system(update_icon)
            .add_system(move_cursor);
    }
}
