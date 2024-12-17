use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{inventory::item::{Item, ItemDatabase}, player::Player, BLOCK_SIZE_PX};

pub struct ItemPickupPlugin;

impl Plugin for ItemPickupPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnItemPickup>();

        app
            .add_systems(Update, spawn_item_pickup)
            .add_systems(FixedUpdate, pull_to_player);
    }
}

#[derive(Event)]
pub struct SpawnItemPickup {
    pub item: Item,
    pub position: Vec2,
}

#[derive(Component)]
pub struct ItemPickup {
    item: Item
}

fn spawn_item_pickup(
    mut commands: Commands,
    mut ev_spawn_item_pickup: EventReader<SpawnItemPickup>,
    item_database: Res<ItemDatabase>,
    asset_server: Res<AssetServer>,
) {
    for ev in ev_spawn_item_pickup.read() {
        commands.spawn((
            Transform {
                translation: ev.position.extend(1.0),
                ..default()
            },
            RigidBody::Dynamic,
            Velocity::zero(),
            LockedAxes::ROTATION_LOCKED,
            Ccd::enabled(),
            Sprite::from_image(asset_server.load(item_database.get_texture_by_id(ev.item.id))),
            Collider::ball(BLOCK_SIZE_PX/2.),
            CollisionGroups::new(Group::GROUP_3, Group::GROUP_2),
            ItemPickup { item: ev.item }
        ));     
    }
}

fn pull_to_player(
    mut commands: Commands,
    q_player: Query<&Transform, (With<Player>, Without<ItemPickup>)>,
    mut q_pickup: Query<(Entity, &mut Velocity, &Transform, &ItemPickup), Without<Player>>,
    time: Res<Time>,
) {
    let Ok(player_transform) = q_player.get_single() else { return };

    for (pickup_entity, mut pickup_velocity, pickup_transform, _item_pickup) in q_pickup.iter_mut() {
        let distance = pickup_transform.translation.distance(player_transform.translation);

        if distance < BLOCK_SIZE_PX * 6. {
            let direction = (player_transform.translation - pickup_transform.translation).truncate().normalize();
            pickup_velocity.linvel = direction * 10_000.0 * time.delta_secs();   
        }

        if distance < BLOCK_SIZE_PX {
            commands.entity(pickup_entity).despawn_recursive();
        }
    }
}