pub mod systems;
use crate::prelude::*;
use systems::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .configure_set(
                Update,
                MovementSystemSet.before(ConfinementSystemSet),
            )
            .add_systems(
                Update,
                (
                    move_camera.in_set(MovementSystemSet),
                    zoom_camera.in_set(MovementSystemSet),
                    bound_camera.in_set(ConfinementSystemSet),
                ),
            );
    }
}
