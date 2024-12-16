use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{BLOCK_SIZE_PX, CHUNK_SIZE};

mod movement;
use movement::PlayerMovementPlugin;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
        app.add_plugins(PlayerMovementPlugin);
    }
}

#[derive(Component)]
#[require(RigidBody(dynamic_rb), LockedAxes(rotation_locked), Collider(box_collider), GravityScale, Velocity)]
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

fn box_collider() -> Collider {
    Collider::cuboid(0.5, 0.5)
}

fn spawn_player(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn((
        Player::default(),
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(materials.add(Color::WHITE)),
        Transform {
            translation: Vec3::new(0.0, BLOCK_SIZE_PX * CHUNK_SIZE as f32, 0.0),
            scale: Vec3::new(BLOCK_SIZE_PX*2., BLOCK_SIZE_PX*3., 1.),
            ..default()
        },
        Friction::coefficient(0.0)
    ));
}