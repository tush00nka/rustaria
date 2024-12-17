use bevy::prelude::*;

use crate::player::Player;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_camera)
            .add_systems(Update, follow_player);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d::default(),
        Msaa::Off,
        OrthographicProjection {
            scale: 0.5,
            ..OrthographicProjection::default_2d()
        },
    ));
    commands.insert_resource(ClearColor(Color::hsl(178., 0.45, 0.43)));
}

fn follow_player(
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
    player_query: Query<&Transform, (With<Player>, Without<Camera>)>,
    // time: Res<Time>,
) {
    let Ok(mut camera_transform) = camera_query.get_single_mut() else { return };
    let Ok(player_transform) = player_query.get_single() else {  return };

    // camera_transform.translation = camera_transform.translation.lerp(player_transform.translation.with_z(camera_transform.translation.z), 10.0 * time.delta_secs());
    camera_transform.translation = player_transform.translation.with_z(camera_transform.translation.z);
}