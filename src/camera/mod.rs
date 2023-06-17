pub mod systems;
use crate::prelude::*;
use systems::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_camera)
            .configure_set(CameraMovementSet.before(CameraConfinementSet))
            .add_systems((
                move_camera.in_set(CameraMovementSet),
                zoom_camera.in_set(CameraMovementSet),
                bound_camera.in_set(CameraConfinementSet),
            ));
    }
}
