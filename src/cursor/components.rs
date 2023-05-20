use bevy::prelude::*;

#[derive(Component)]
pub struct GameCursor();

#[derive(Resource)]
pub struct Cursors {
    pub point: Handle<Image>,
    pub click: Handle<Image>,
}

// impl FromWorld for Cursors {
//   fn from_world(world: &mut World) -> Self {
//     let asset_server = world.resource_mut::<AssetServer>();
//     Cursors {
//       point: asset_server.load("cursor/point/path"),
//       click: asset_server.load("cursor/click/path"),
//     }
//   }
// }

// #[derive(Component, Clone, Default)]
// pub struct UiCursor {
//   pub image: Handle<Image>,
//   pub size: Vec2,
// }

// #[derive(Bundle, Clone, Default)]
// pub struct UiCursorBundle {
//     pub(crate) cursor: UiCursor,
//     pub(crate) transform: Transform,
//     pub(crate) global_transfrom: GlobalTransform,
//     pub(crate) visibility: Visibility,
//     pub(crate) computed_visibility: ComputedVisibility
// }