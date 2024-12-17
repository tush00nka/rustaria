use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{inventory::Inventory, BLOCK_SIZE_PX, CHUNK_SIZE};

mod movement;
use movement::PlayerMovementPlugin;

mod block_interaction;
use block_interaction::BlockInteractionPlugin;

mod hotbar;
use hotbar::HotbarPlugin;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
        app.add_plugins((PlayerMovementPlugin, BlockInteractionPlugin, HotbarPlugin));
    }
}

#[derive(Component)]
#[require(RigidBody(dynamic_rb), LockedAxes(rotation_locked), Collider(collider), GravityScale, Velocity)]
pub struct Player {
    speed: f32,
    jump_force: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            speed: 10000.0,
            jump_force: 350.0,
        }
    }
}

fn dynamic_rb() -> RigidBody {
    RigidBody::Dynamic
}

fn rotation_locked() -> LockedAxes {
    LockedAxes::ROTATION_LOCKED
}

fn collider() -> Collider {
    Collider::capsule(Vec2::new(0.0, -0.25), Vec2::new(0.0, 0.25), 0.25)
}

fn spawn_player(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn((
        Player::default(),
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(materials.add(Color::WHITE)),
        Transform {
            translation: Vec3::new(0.0, BLOCK_SIZE_PX * CHUNK_SIZE as f32, 0.0),
            scale: Vec3::new(BLOCK_SIZE_PX, BLOCK_SIZE_PX*2., 1.),
            ..default()
        },
        Ccd::enabled(),
        CollisionGroups::new(Group::GROUP_1, Group::GROUP_2),
        Friction::coefficient(0.0),
        Inventory::new(9)
    ));
}