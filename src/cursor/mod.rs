mod components;
mod systems;
use self::systems::*;
use crate::GameState;
use ::bevy::prelude::*;

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_cursor_icons)
            .add_system(
                spawn_menu_cursor.in_schedule(OnEnter(GameState::MainMenu)),
            )
            .add_system(update_icon)
            //.add_system(despawn_cursor.in_schedule(OnExit(GameState::MainMenu)))
            .add_system(move_cursor);
    }
}

// impl Plugin for CursorPlugin {
//     fn build(&self, app: &mut App) {
//         let render_app = match app.get_sub_app_mut(RenderApp) {
//             Ok(render_app) => render_app,
//             Err(_) => return,
//         };

//         render_app
//             .add_system(
//                 extract_ui_cursor
//                     .after(RenderUiSystem::ExtractNode)
//                     .in_schedule(ExtractSchedule),
//             );

//         app
//             .init_resource::<Cursors>()
//             .add_startup_system(setup_cursor_icons)
//             .add_startup_system(spawn_ui_cursor);
//     }
// }
