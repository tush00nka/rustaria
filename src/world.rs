use std::collections::HashMap;

use bevy::prelude::*;

pub mod chunk;
use block::{Block, BlockDatabase, BlockLayer};
use chunk::*;

use crate::{BLOCK_SIZE_PX, CHUNK_SIZE};

pub struct WorldPlugin;
impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ChunkPlugin);

        app.init_resource::<World>();
        app.add_event::<SetBlock>();

        app
            .add_systems(PostStartup, (generate_world_data, draw_world.after(generate_world_data)))
            .add_systems(Update, set_block_at_position);
    }
}

#[derive(Resource, Default)]
pub struct World {
    chunks: HashMap<(i32, i32), Chunk>,
    chunk_entites: HashMap<(i32, i32), Entity>,
}

impl World {
    #[allow(unused)]
    pub fn get_chunk(&self, x: i32, y: i32) -> Option<&Chunk> {
        if self.chunks.contains_key(&(x,y)) {
            return Some(self.chunks.get(&(x,y)).unwrap());
        }

        None
    }

    pub fn get_chunk_mut(&mut self, x: i32, y: i32) -> Option<&mut Chunk> {
        if self.chunks.contains_key(&(x,y)) {
            return Some(self.chunks.get_mut(&(x,y)).unwrap());
        }

        None
    }
}

fn generate_world_data(mut world: ResMut<World>, block_database: Res<BlockDatabase>) {
    for y in -8..1 {
        for x in -8..8 {
            let mut chunk = Chunk::new(x, y);

            chunk.fill_block_data(ChunkBlockPool {
                grass: block_database.get_by_id(2),
                dirt: block_database.get_by_id(1),
                stone: block_database.get_by_id(3)
            });
            world.chunks.insert((x,y), chunk);
        }
    }
}

fn draw_world(
    world: Res<World>,
    mut ev_draw_chunk: EventWriter<DrawChunk>,
) {
    for (_, chunk) in world.chunks.iter() {
        ev_draw_chunk.send(DrawChunk { chunk: *chunk });
    }
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
    mut ev_draw_chunk: EventWriter<DrawChunk>,
) {
    for ev in ev_break_block.read() {
        let (chunk_x, chunk_y) = ((ev.position.x / CHUNK_SIZE as f32 / BLOCK_SIZE_PX).floor() as i32,
                                            (ev.position.y / CHUNK_SIZE as f32 / BLOCK_SIZE_PX).floor() as i32);

        let Some(chunk) = world.get_chunk_mut(chunk_x, chunk_y) else { return; };

        let (block_x, block_y) = ((ev.position.x / BLOCK_SIZE_PX - (chunk_x as f32 * CHUNK_SIZE as f32)) as usize,
                                                (ev.position.y / BLOCK_SIZE_PX - (chunk_y as f32 * CHUNK_SIZE as f32)) as usize);


        match ev.layer {
            BlockLayer::Foreground => { 
                if !ev.can_overwrite && chunk.data[block_x][block_y].id != 0 {
                    return;
                }
                chunk.data[block_x][block_y] = ev.block;
            },
            BlockLayer::Background => { 
                if !ev.can_overwrite && chunk.background_data[block_x][block_y].id != 0 {
                    return;
                }
                chunk.background_data[block_x][block_y] = ev.block
            }
        }

        ev_draw_chunk.send(DrawChunk { chunk: *chunk });
    }
}