use bevy::prelude::*;

pub struct MousePositionPlugin;

impl Plugin for MousePositionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MousePosition>();

        app.add_systems(Update, update_mouse_position);
    }
}

#[derive(Resource, Default)]
pub struct MousePosition(pub Vec2);

fn update_mouse_position(
    mut mouse_position: ResMut<MousePosition>,
    window: Single<&Window>,
    camera: Single<(&Camera, &GlobalTransform)>, 
) {
    let (camera, camera_transform) = camera.into_inner();
    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor).ok()) {
            mouse_position.0 = world_position;
        }
}