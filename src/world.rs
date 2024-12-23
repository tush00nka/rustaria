use bevy::prelude::*;

pub mod chunk;
use block::{Block, BlockDatabase, BlockLayer};
use chunk::*;

use crate::{
    inventory::item::ItemDatabase,
    item_pickup::SpawnItemPickup,
    BLOCK_SIZE_PX,
    WORLD_HEIGHT,
    WORLD_WIDTH
};

pub struct WorldPlugin;
impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ChunkPlugin);

        app.insert_resource(World {
            chunk: Chunk::new(),
            chunk_entity: Entity::PLACEHOLDER
        });
        app.add_event::<SetBlock>();

        app
            .add_systems(Startup, generate_world)
            .add_systems(Update, set_block_at_position);
    }
}

#[derive(Resource)]
pub struct World {
    pub chunk: Chunk,
    pub chunk_entity: Entity,
}

impl World {

    pub fn get_block(&self, x: f32, y: f32, layer: BlockLayer) -> Option<Block> {
        let (chunk_x, chunk_y) = ((x / WORLD_WIDTH as f32 / BLOCK_SIZE_PX).floor() as i32,
                                            (y / WORLD_HEIGHT as f32 / BLOCK_SIZE_PX).floor() as i32);

        let (block_x, block_y) = ((x / BLOCK_SIZE_PX - (chunk_x as f32 * WORLD_WIDTH as f32)) as usize,
                                                (y / BLOCK_SIZE_PX - (chunk_y as f32 * WORLD_HEIGHT as f32)) as usize);

        match layer {
            BlockLayer::Foreground => {
                return Some(self.chunk.data[block_x][block_y]);
            },
            BlockLayer::Background => {
                return Some(self.chunk.background_data[block_x][block_y]);
            }
        }
        
    }
}

fn generate_world (
    mut ev_generate_chunk_data: EventWriter<GenerateChunkData>
) {
    ev_generate_chunk_data.send(GenerateChunkData);

}

#[derive(Event)]
pub struct SetBlock{
    pub block: Block,
    pub position: Vec2,
    pub layer: BlockLayer,
    pub can_overwrite: bool,
}

fn set_block_at_position(
    mut ev_break_block: EventReader<SetBlock>,
    mut world: ResMut<World>,
    mut ev_update_light: EventWriter<UpdateChunkLight>,
    mut ev_spawn_item_pickup: EventWriter<SpawnItemPickup>,
    item_database: Res<ItemDatabase>,
    block_database: Res<BlockDatabase>,
) {
    for ev in ev_break_block.read() {
        let (chunk_x, chunk_y) = ((ev.position.x / WORLD_WIDTH as f32 / BLOCK_SIZE_PX).floor() as i32,
                                            (ev.position.y / WORLD_WIDTH as f32 / BLOCK_SIZE_PX).floor() as i32);

        let chunk = &mut world.chunk;

        let (block_x, block_y) = ((ev.position.x / BLOCK_SIZE_PX - (chunk_x as f32 * WORLD_WIDTH as f32)) as usize,
                                                (ev.position.y / BLOCK_SIZE_PX - (chunk_y as f32 * WORLD_HEIGHT as f32)) as usize);

        let block_to_replace;

        match ev.layer {
            BlockLayer::Foreground => { 
                if !ev.can_overwrite && chunk.data[block_x][block_y].id != 0 {
                    return;
                }
                block_to_replace = chunk.data[block_x][block_y];
                chunk.data[block_x][block_y] = ev.block;
            },
            BlockLayer::Background => { 
                if !ev.can_overwrite && chunk.background_data[block_x][block_y].id != 0 {
                    return;
                }
                block_to_replace = chunk.background_data[block_x][block_y];
                chunk.background_data[block_x][block_y] = ev.block;
            }
        }

        if ev.block.id == 0 && block_to_replace.id != 0 {
            ev_spawn_item_pickup.send(SpawnItemPickup {
                item: item_database.get_by_id(block_database.get_by_id(block_to_replace.id).drop_item),
                position: Vec2::new(ev.position.x + BLOCK_SIZE_PX/2., ev.position.y + BLOCK_SIZE_PX/2.),
            });
        }

        ev_update_light.send(UpdateChunkLight { chunk: chunk.clone() });
    }
}