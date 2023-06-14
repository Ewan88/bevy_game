mod systems;
use crate::prelude::*;
use systems::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_camera)
            .configure_set(MovementSystemSet.before(ConfinementSystemSet))
            .add_systems((
                move_camera.in_set(MovementSystemSet),
                bound_camera.in_set(ConfinementSystemSet),
            ));
    }
}
