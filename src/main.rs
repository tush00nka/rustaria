use bevy::{
        prelude::*,
        window::WindowResolution
};

use bevy_rapier2d::prelude::*;

mod world;
use world::WorldPlugin;

mod camera;
use camera::CameraPlugin;

mod player;
use player::PlayerPlugin;

mod mouse_position;
use mouse_position::MousePositionPlugin;

mod inventory;
use inventory::InventoryPlugin;

mod item_pickup;
use item_pickup::ItemPickupPlugin;

mod ui;
use ui::UiPlugin;

const CHUNK_WIDTH: usize = 32;
const CHUNK_HEIGHT: usize = 256;
const BLOCK_SIZE_PX: f32 = 16.;

const SEED: &'static str = "I WANNA KMS";

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "rustaria".to_string(),
                        resizable: false,
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        resolution: WindowResolution::new(1280., 720.),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        // .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(WorldPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(MousePositionPlugin)
        .add_plugins(InventoryPlugin)
        .add_plugins(ItemPickupPlugin)
        .add_plugins(UiPlugin)
        .run();
}     

